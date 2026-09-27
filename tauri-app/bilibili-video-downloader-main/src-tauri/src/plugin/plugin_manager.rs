use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use eyre::eyre;
use parking_lot::RwLock;
use tauri::{AppHandle, Manager};
use tauri_specta::Event;
use tracing::instrument;

use crate::{
    events::PluginEvent,
    extensions::EyreReportToMessage,
    types::plugin_info::{PluginDescriptorInfo, PluginInfo, PluginMetadata, PluginRuntimeStatus},
};

use super::{
    hook_context::HookContext, host_api, plugin_executor, plugin_loader,
    plugin_types::PluginRuntime,
};

pub struct PluginManager {
    app: AppHandle,
    infos: RwLock<HashMap<String, PluginInfo>>,
    runtimes: RwLock<Vec<PluginRuntime>>,
}

impl PluginManager {
    #[instrument(level = "error", skip_all)]
    pub fn new(app: &AppHandle) -> eyre::Result<PluginManager> {
        host_api::init(app);

        let app_data_dir = app.path().app_data_dir()?;
        let plugin_json_path = app_data_dir.join("plugin.json");

        let mut infos = HashMap::new();
        if plugin_json_path.exists() {
            let json_string = std::fs::read_to_string(&plugin_json_path)?;

            let metadata_map: HashMap<String, PluginMetadata> =
                serde_json::from_str(&json_string).unwrap_or_default();

            for (plugin_path, metadata) in metadata_map {
                let status = PluginRuntimeStatus::Unknown;
                let info = PluginInfo::from_metadata(metadata, status);
                infos.insert(plugin_path, info);
            }
        }

        let mut runtimes = Vec::new();
        for info in infos.values_mut() {
            if !info.enabled {
                info.runtime_status = PluginRuntimeStatus::Disabled;
                continue;
            }

            match plugin_loader::load_plugin_from_path(&info.path, info.priority, true) {
                Ok(runtime) => {
                    tracing::info!(
                        "插件加载成功: plugin_name={}, plugin_path={}",
                        runtime.display_name(),
                        runtime.plugin_path.display()
                    );

                    info.runtime_status = PluginRuntimeStatus::Loaded;
                    info.descriptor = PluginDescriptorInfo::from_descriptor(&runtime.descriptor);

                    insert_runtime_by_priority(&mut runtimes, runtime);
                }
                Err(err) => {
                    let err_title = "某个插件加载失败，已跳过";
                    let message = err.to_message();
                    tracing::error!(err_title, message);

                    info.runtime_status = PluginRuntimeStatus::LoadFailed;
                }
            }
        }

        let plugin_manager = Self {
            app: app.clone(),
            infos: RwLock::new(infos),
            runtimes: RwLock::new(runtimes),
        };

        plugin_manager.save_metadata()?;

        Ok(plugin_manager)
    }

    #[instrument(level = "error", skip_all, fields(plugin_path = plugin_path))]
    pub fn add_plugin(&self, plugin_path: &str) -> eyre::Result<()> {
        let runtime = plugin_loader::load_plugin_from_path(&PathBuf::from(plugin_path), 0, true)?;

        let plugin_info = {
            let mut infos = self.infos.write();
            if infos.contains_key(plugin_path) {
                return Err(eyre!("插件已存在: {plugin_path}"));
            }
            let status = PluginRuntimeStatus::Loaded;
            let metadata = PluginMetadata::from_plugin_runtime(&runtime);
            let info = PluginInfo::from_metadata(metadata, status);
            infos.insert(plugin_path.to_string(), info.clone());
            info
        };

        {
            let mut runtimes = self.runtimes.write();
            insert_runtime_by_priority(&mut runtimes, runtime);
        }

        self.save_metadata()?;
        let _ = PluginEvent::Loaded { plugin_info }.emit(&self.app);

        Ok(())
    }

    #[instrument(level = "error", skip_all, fields(plugin_path = plugin_path))]
    pub fn uninstall_plugin(&self, plugin_path: &str) -> eyre::Result<()> {
        {
            let mut infos = self.infos.write();
            if !infos.contains_key(plugin_path) {
                return Err(eyre!("key中没有插件路径: {plugin_path}"));
            }
            infos.remove(plugin_path);
        }

        {
            let mut runtimes = self.runtimes.write();
            remove_runtime_by_path(&mut runtimes, Path::new(plugin_path));
        }

        let _ = PluginEvent::Uninstall {
            plugin_path: plugin_path.to_string(),
        }
        .emit(&self.app);

        self.save_metadata()?;

        Ok(())
    }

    #[instrument(level = "error", skip_all, fields(plugin_path = plugin_path, enabled = enabled))]
    pub fn set_plugin_enabled(&self, plugin_path: &str, enabled: bool) -> eyre::Result<()> {
        if !enabled {
            let plugin_info = {
                let mut infos = self.infos.write();
                let Some(info) = infos.get_mut(plugin_path) else {
                    return Err(eyre!("key中没有插件路径: {plugin_path}"));
                };
                if info.enabled == enabled {
                    return Ok(());
                }
                info.enabled = false;
                info.runtime_status = PluginRuntimeStatus::Disabled;
                info.clone()
            };

            {
                let mut runtimes = self.runtimes.write();
                remove_runtime_by_path(&mut runtimes, Path::new(plugin_path));
            }

            let _ = PluginEvent::Update { plugin_info }.emit(&self.app);

            self.save_metadata()?;

            return Ok(());
        }

        let (plugin_file_path, priority) = {
            let mut infos = self.infos.write();
            let Some(info) = infos.get_mut(plugin_path) else {
                return Err(eyre!("key中没有插件路径: {plugin_path}"));
            };
            if info.enabled == enabled {
                return Ok(());
            }
            info.enabled = true;
            (info.path.clone(), info.priority)
        };

        let plugin_info =
            match plugin_loader::load_plugin_from_path(&plugin_file_path, priority, true) {
                Ok(runtime) => {
                    {
                        let mut runtimes = self.runtimes.write();
                        remove_runtime_by_path(&mut runtimes, &plugin_file_path);
                        insert_runtime_by_priority(&mut runtimes, runtime.clone());
                    }

                    let mut infos = self.infos.write();
                    let Some(info) = infos.get_mut(plugin_path) else {
                        return Err(eyre!("key中没有插件路径: {plugin_path}"));
                    };
                    info.runtime_status = PluginRuntimeStatus::Loaded;
                    info.descriptor = PluginDescriptorInfo::from_descriptor(&runtime.descriptor);
                    info.clone()
                }
                Err(err) => {
                    let err_title = "启用插件时加载失败";
                    let message = err.to_message();
                    tracing::error!(err_title, message);

                    {
                        let mut runtimes = self.runtimes.write();
                        remove_runtime_by_path(&mut runtimes, &plugin_file_path);
                    }

                    let mut infos = self.infos.write();
                    let Some(info) = infos.get_mut(plugin_path) else {
                        return Err(eyre!("key中没有插件路径: {plugin_path}"));
                    };
                    info.runtime_status = PluginRuntimeStatus::LoadFailed;
                    info.clone()
                }
            };

        let _ = PluginEvent::Update { plugin_info }.emit(&self.app);

        self.save_metadata()?;

        Ok(())
    }

    #[instrument(
        level = "error",
        skip_all,
        fields(plugin_path = plugin_path, priority = priority)
    )]
    pub fn set_plugin_priority(&self, plugin_path: &str, priority: i32) -> eyre::Result<()> {
        let plugin_info = {
            let mut infos = self.infos.write();
            let Some(info) = infos.get_mut(plugin_path) else {
                return Err(eyre!("key中没有插件路径: {plugin_path}"));
            };
            if info.priority == priority {
                return Ok(());
            }
            info.priority = priority;
            info.clone()
        };

        {
            let mut runtimes = self.runtimes.write();
            if let Some(mut runtime) = remove_runtime_by_path(&mut runtimes, Path::new(plugin_path))
            {
                runtime.priority = priority;
                insert_runtime_by_priority(&mut runtimes, runtime);
            }
        }

        let _ = PluginEvent::Update { plugin_info }.emit(&self.app);

        self.save_metadata()?;

        Ok(())
    }

    pub fn get_plugin_infos(&self) -> Vec<PluginInfo> {
        self.infos.read().values().cloned().collect()
    }

    #[instrument(level = "error", skip_all)]
    pub async fn run_hook(&self, mut context: HookContext<'_>) -> eyre::Result<()> {
        let hook_point = context.hook_point();
        let runtimes = self.runtimes.read().clone();
        if runtimes.is_empty() {
            return Ok(());
        }

        let app_version = self.app.package_info().version.to_string();

        for runtime in &runtimes {
            if !runtime.enabled || !runtime.should_run_hook(hook_point) {
                continue;
            }

            let input = context.to_input(&app_version)?;
            let output = match plugin_executor::execute_hook(runtime, &input).await {
                Ok(output) => output,
                Err(err) => match runtime.descriptor.failure_policy {
                    bilibili_video_downloader_plugin_api::v1::PluginFailurePolicy::FailOpen => {
                        let err_title = "插件执行出错，按照 FailOpen 继续其他任务";
                        let message = err.to_message();
                        tracing::error!(err_title, message);
                        continue;
                    }
                    bilibili_video_downloader_plugin_api::v1::PluginFailurePolicy::FailClosed => {
                        let err = err.wrap_err("插件执行出错，按照 FailClosed 中断任务");
                        return Err(err);
                    }
                },
            };

            if let Err(err) = context.apply_output(output) {
                match runtime.descriptor.failure_policy {
                    bilibili_video_downloader_plugin_api::v1::PluginFailurePolicy::FailOpen => {
                        let err_title = "插件输出无效，按照 FailOpen 继续其他任务";
                        let message = err.to_message();
                        tracing::error!(err_title, message);
                    }
                    bilibili_video_downloader_plugin_api::v1::PluginFailurePolicy::FailClosed => {
                        let err = err.wrap_err("插件输出无效，按照 FailClosed 中断任务");
                        return Err(err);
                    }
                }
            }
        }

        Ok(())
    }

    #[instrument(level = "error", skip_all)]
    fn save_metadata(&self) -> eyre::Result<()> {
        let app_data_dir = self.app.path().app_data_dir()?;
        let plugin_json_path = app_data_dir.join("plugin.json");

        let metadata_by_path: HashMap<String, PluginMetadata> = self
            .infos
            .read()
            .clone()
            .into_iter()
            .map(|(plugin_path, info)| (plugin_path, info.into_metadata()))
            .collect();
        let json_string = serde_json::to_string_pretty(&metadata_by_path)?;

        std::fs::write(plugin_json_path, json_string)?;

        Ok(())
    }
}

fn insert_runtime_by_priority(runtimes: &mut Vec<PluginRuntime>, runtime: PluginRuntime) {
    let insert_idx = runtimes
        .iter()
        .position(|existing| existing.priority < runtime.priority)
        .unwrap_or(runtimes.len());
    runtimes.insert(insert_idx, runtime);
}

fn remove_runtime_by_path(
    runtimes: &mut Vec<PluginRuntime>,
    plugin_path: &Path,
) -> Option<PluginRuntime> {
    let remove_idx = runtimes
        .iter()
        .position(|runtime| runtime.plugin_path == plugin_path)?;
    Some(runtimes.remove(remove_idx))
}
