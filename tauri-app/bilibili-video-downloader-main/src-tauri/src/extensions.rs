use eyre::WrapErr;
use parking_lot::RwLock;
use tauri::{AppHandle, Manager, State};
use tracing::instrument;

use crate::{
    bili_client::BiliClient,
    config::Config,
    downloader::{download_manager::DownloadManager, download_progress::DownloadProgress},
    plugin::plugin_manager::PluginManager,
    types::player_info::PlayerInfo,
};

pub trait EyreReportToMessage {
    fn to_message(&self) -> String;
}

impl EyreReportToMessage for eyre::Report {
    fn to_message(&self) -> String {
        format!("{self:?}")
    }
}

pub trait AppHandleExt {
    fn get_config(&self) -> State<'_, RwLock<Config>>;
    fn get_bili_client(&self) -> State<'_, BiliClient>;
    fn get_download_manager(&self) -> State<'_, DownloadManager>;
    fn get_plugin_manager(&self) -> State<'_, PluginManager>;
}

impl AppHandleExt for AppHandle {
    fn get_config(&self) -> State<'_, RwLock<Config>> {
        self.state::<RwLock<Config>>()
    }
    fn get_bili_client(&self) -> State<'_, BiliClient> {
        self.state::<BiliClient>()
    }
    fn get_download_manager(&self) -> State<'_, DownloadManager> {
        self.state::<DownloadManager>()
    }
    fn get_plugin_manager(&self) -> State<'_, PluginManager> {
        self.state::<PluginManager>()
    }
}

pub trait GetOrInitPlayerInfo {
    async fn get_or_init<'a>(
        &'a mut self,
        app: &AppHandle,
        progress: &DownloadProgress,
    ) -> eyre::Result<&'a mut PlayerInfo>;
}

impl GetOrInitPlayerInfo for Option<PlayerInfo> {
    #[instrument(level = "error", skip_all)]
    async fn get_or_init<'a>(
        &'a mut self,
        app: &AppHandle,
        progress: &DownloadProgress,
    ) -> eyre::Result<&'a mut PlayerInfo> {
        if let Some(info) = self {
            return Ok(info);
        }

        let bili_client = app.get_bili_client();
        let info = bili_client
            .get_player_info(progress.aid, progress.cid)
            .await
            .wrap_err("获取播放器信息失败")?;

        Ok(self.insert(info))
    }
}
