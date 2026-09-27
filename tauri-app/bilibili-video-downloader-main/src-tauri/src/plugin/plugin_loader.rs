use std::{ffi::CStr, path::Path, sync::Arc};

use bilibili_video_downloader_plugin_api::{SDK_API_VERSION_V1, v1::PluginDescriptorV1};
use dlopen2::wrapper::Container;
use eyre::{WrapErr, eyre};
use tracing::instrument;

use crate::plugin::{
    host_api,
    plugin_types::{PluginDylibApi, PluginRuntime},
};

#[instrument(level = "error", skip_all, fields(plugin_path = %plugin_path.display(), priority = priority, enabled = enabled))]
pub fn load_plugin_from_path(
    plugin_path: &Path,
    priority: i32,
    enabled: bool,
) -> eyre::Result<PluginRuntime> {
    if !plugin_path.is_absolute() {
        return Err(eyre!("插件路径必须是绝对路径: `{}`", plugin_path.display()));
    }
    if !plugin_path.exists() {
        return Err(eyre!("插件动态库文件`{}`不存在", plugin_path.display()));
    }

    let api = unsafe { Container::<PluginDylibApi>::load(plugin_path) }
        .wrap_err(format!("加载插件动态库文件`{}`失败", plugin_path.display()))?;

    let descriptor_json = get_descriptor_json(&api).wrap_err("读取插件描述失败")?;
    let descriptor: PluginDescriptorV1 = serde_json::from_str(&descriptor_json)
        .wrap_err(format!("解析插件描述失败: {descriptor_json}"))?;

    if descriptor.sdk_api_version != SDK_API_VERSION_V1 {
        return Err(eyre!(
            "插件SDK版本不匹配: 期望版本={}, 实际版本={}",
            SDK_API_VERSION_V1,
            descriptor.sdk_api_version
        ));
    }

    if descriptor.id.trim().is_empty() {
        return Err(eyre!("descriptor.id 为空"));
    }

    if descriptor.hooks.is_empty() {
        return Err(eyre!("插件未声明任何可执行 Hook"));
    }

    let host_api = host_api::build_host_api_v1();
    let rc = unsafe { api.set_host_api(&raw const host_api) };
    if rc != 0 {
        return Err(eyre!(
            "注册宿主 Host API 失败: plugin_id={}, rc={rc}",
            descriptor.id
        ));
    }

    Ok(PluginRuntime {
        descriptor,
        plugin_path: plugin_path.to_path_buf(),
        enabled,
        priority,
        api: Arc::new(api),
    })
}

#[instrument(level = "error", skip_all)]
fn get_descriptor_json(api: &Container<PluginDylibApi>) -> eyre::Result<String> {
    let descriptor_ptr = unsafe { api.descriptor() };
    if descriptor_ptr.is_null() {
        return Err(eyre!("descriptor 指针为空"));
    }

    let descriptor_cstr = unsafe { CStr::from_ptr(descriptor_ptr).to_str() }
        .wrap_err("descriptor 非 UTF-8 字符串")?;

    Ok(descriptor_cstr.to_string())
}
