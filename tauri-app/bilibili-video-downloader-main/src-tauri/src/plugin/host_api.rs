use std::sync::OnceLock;

use bilibili_video_downloader_plugin_api::v1::{HostApiV1, HostConfigV1};
use eyre::WrapErr;
use tauri::AppHandle;

use crate::{config::Config, extensions::AppHandleExt};

static HOST_APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub fn init(app: &AppHandle) {
    HOST_APP_HANDLE.get_or_init(|| app.clone());
}

pub fn build_host_api_v1() -> HostApiV1 {
    HostApiV1 {
        get_config_json: host_get_config_json_v1,
        free_buffer: host_free_buffer_v1,
    }
}

unsafe extern "C" fn host_get_config_json_v1(out_ptr: *mut *mut u8, out_len: *mut usize) -> i32 {
    if out_ptr.is_null() || out_len.is_null() {
        return 1;
    }

    let Some(app) = HOST_APP_HANDLE.get() else {
        return 2;
    };

    let host_config = app.get_config().read().clone();
    let Ok(host_config_v1) = to_host_config_v1(&host_config) else {
        return 3;
    };

    let Ok(output_bytes) = serde_json::to_vec(&host_config_v1) else {
        return 3;
    };

    let boxed = output_bytes.into_boxed_slice();
    let len = boxed.len();
    let ptr = Box::into_raw(boxed).cast::<u8>();

    unsafe {
        *out_ptr = ptr;
        *out_len = len;
    }

    0
}

unsafe extern "C" fn host_free_buffer_v1(ptr: *mut u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }

    let raw_slice = std::ptr::slice_from_raw_parts_mut(ptr, len);
    unsafe {
        drop(Box::from_raw(raw_slice));
    }
}

fn to_host_config_v1(config: &Config) -> eyre::Result<HostConfigV1> {
    let value = serde_json::to_value(config).wrap_err("序列化宿主 Config 失败")?;
    let host_config = serde_json::from_value(value).wrap_err("反序列化为插件 HostConfigV1 失败")?;
    Ok(host_config)
}
