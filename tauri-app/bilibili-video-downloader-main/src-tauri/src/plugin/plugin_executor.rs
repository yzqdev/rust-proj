use std::{ffi::CStr, sync::Arc};

use bilibili_video_downloader_plugin_api::v1::{HookInputV1, HookOutputV1};
use dlopen2::wrapper::Container;
use eyre::eyre;
use tracing::instrument;

use crate::plugin::plugin_types::{PluginDylibApi, PluginRuntime};

#[instrument(level = "error", skip_all, fields(plugin_name = plugin.display_name(), hook_point = ?input.hook_point))]
pub async fn execute_hook(
    plugin: &PluginRuntime,
    input: &HookInputV1,
) -> eyre::Result<HookOutputV1> {
    let input_bytes = serde_json::to_vec(input)?;
    let api = plugin.api.clone();

    let (tx, rx) = tokio::sync::oneshot::channel::<eyre::Result<Vec<u8>>>();
    tauri::async_runtime::spawn_blocking(move || {
        let result = call_on_hook_blocking(api, &input_bytes);
        let _ = tx.send(result);
    });

    let output_bytes = rx.await??;
    let output: HookOutputV1 = serde_json::from_slice(&output_bytes)?;
    Ok(output)
}

#[instrument(level = "error", skip_all)]
#[allow(clippy::needless_pass_by_value)]
fn call_on_hook_blocking(
    api: Arc<Container<PluginDylibApi>>,
    input_bytes: &[u8],
) -> eyre::Result<Vec<u8>> {
    let mut output_ptr: *mut u8 = std::ptr::null_mut();
    let mut output_len: usize = 0;
    let rc = unsafe {
        api.on_hook(
            input_bytes.as_ptr(),
            input_bytes.len(),
            &raw mut output_ptr,
            &raw mut output_len,
        )
    };
    if rc != 0 {
        let detail = get_last_error(&api);
        return Err(eyre!("插件返回错误码: code={rc}, detail={detail}"));
    }
    if output_ptr.is_null() {
        return Err(eyre!("插件返回空输出缓冲区"));
    }

    let output_bytes = unsafe { std::slice::from_raw_parts(output_ptr, output_len) }.to_vec();
    unsafe { api.free_buffer(output_ptr, output_len) };
    Ok(output_bytes)
}

fn get_last_error(api: &Arc<Container<PluginDylibApi>>) -> String {
    let error_ptr = unsafe { api.last_error() };
    if error_ptr.is_null() {
        return "获取错误信息失败，error_ptr为null".to_string();
    }

    let error_cstr = unsafe { CStr::from_ptr(error_ptr) };
    error_cstr.to_string_lossy().to_string()
}
