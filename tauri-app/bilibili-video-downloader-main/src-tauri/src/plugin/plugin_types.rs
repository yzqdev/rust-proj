use std::{ffi::c_char, path::PathBuf, sync::Arc};

use bilibili_video_downloader_plugin_api::v1::{HookPointV1, HostApiV1, PluginDescriptorV1};
use dlopen2::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
pub struct PluginDylibApi {
    #[dlopen2_name = "bilibili_video_downloader_plugin_descriptor_v1"]
    descriptor: unsafe extern "C" fn() -> *const c_char,
    #[dlopen2_name = "bilibili_video_downloader_plugin_on_hook_v1"]
    on_hook: unsafe extern "C" fn(
        input_ptr: *const u8,
        input_len: usize,
        out_ptr: *mut *mut u8,
        out_len: *mut usize,
    ) -> i32,
    #[dlopen2_name = "bilibili_video_downloader_plugin_free_buffer_v1"]
    free_buffer: unsafe extern "C" fn(ptr: *mut u8, len: usize),
    #[dlopen2_name = "bilibili_video_downloader_plugin_last_error_v1"]
    last_error: unsafe extern "C" fn() -> *const c_char,
    #[dlopen2_name = "bilibili_video_downloader_plugin_set_host_api_v1"]
    set_host_api: unsafe extern "C" fn(api: *const HostApiV1) -> i32,
}

#[derive(Clone)]
pub struct PluginRuntime {
    pub descriptor: PluginDescriptorV1,
    pub plugin_path: PathBuf,
    pub enabled: bool,
    pub priority: i32,
    pub api: Arc<Container<PluginDylibApi>>,
}

impl PluginRuntime {
    pub fn display_name(&self) -> String {
        format!(
            "{} ({}, v{})",
            self.descriptor.name, self.descriptor.id, self.descriptor.version
        )
    }

    pub fn should_run_hook(&self, hook: HookPointV1) -> bool {
        self.descriptor.hooks.contains(&hook)
    }
}
