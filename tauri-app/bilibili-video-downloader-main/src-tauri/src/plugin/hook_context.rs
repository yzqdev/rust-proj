use bilibili_video_downloader_plugin_api::v1::{
    AfterPreparePayloadV1, BeforeVideoProcessPayloadV1, DownloadProgressV1, HookInputV1,
    HookOutputV1, HookPayloadV1, HookPointV1, HookReadonlyMetaV1, OnCompletedPayloadV1,
};
use eyre::{WrapErr, eyre};
use serde::{Serialize, de::DeserializeOwned};

use crate::downloader::download_progress::DownloadProgress;

pub struct BeforeVideoProcessContext<'a> {
    progress: &'a mut DownloadProgress,
}

impl<'a> BeforeVideoProcessContext<'a> {
    pub fn new(progress: &'a mut DownloadProgress) -> Self {
        Self { progress }
    }

    fn to_payload(&self) -> eyre::Result<BeforeVideoProcessPayloadV1> {
        Ok(BeforeVideoProcessPayloadV1 {
            progress: host_to_api_progress(self.progress)?,
        })
    }

    fn apply_payload(&mut self, payload: BeforeVideoProcessPayloadV1) -> eyre::Result<()> {
        validate_task_id_unchanged(self.progress, &payload.progress)?;

        let next_progress = api_to_host_progress(payload.progress)?;

        *self.progress = next_progress;

        Ok(())
    }
}

pub struct OnCompletedContext<'a> {
    progress: &'a mut DownloadProgress,
}

impl<'a> OnCompletedContext<'a> {
    pub fn new(progress: &'a mut DownloadProgress) -> Self {
        Self { progress }
    }

    fn to_payload(&self) -> eyre::Result<OnCompletedPayloadV1> {
        Ok(OnCompletedPayloadV1 {
            progress: host_to_api_progress(self.progress)?,
        })
    }

    fn apply_payload(&mut self, payload: OnCompletedPayloadV1) -> eyre::Result<()> {
        validate_task_id_unchanged(self.progress, &payload.progress)?;

        let next_progress = api_to_host_progress(payload.progress)?;

        *self.progress = next_progress;

        Ok(())
    }
}

pub struct AfterPrepareContext<'a> {
    progress: &'a mut DownloadProgress,
}

impl<'a> AfterPrepareContext<'a> {
    pub fn new(progress: &'a mut DownloadProgress) -> Self {
        Self { progress }
    }

    fn to_payload(&self) -> eyre::Result<AfterPreparePayloadV1> {
        Ok(AfterPreparePayloadV1 {
            progress: host_to_api_progress(self.progress)?,
        })
    }

    fn apply_payload(&mut self, payload: AfterPreparePayloadV1) -> eyre::Result<()> {
        validate_task_id_unchanged(self.progress, &payload.progress)?;

        let next_progress = api_to_host_progress(payload.progress)?;

        *self.progress = next_progress;

        Ok(())
    }
}

pub enum HookContext<'a> {
    BeforeVideoProcess(BeforeVideoProcessContext<'a>),
    AfterPrepare(AfterPrepareContext<'a>),
    OnCompleted(OnCompletedContext<'a>),
}

impl HookContext<'_> {
    pub fn hook_point(&self) -> HookPointV1 {
        match self {
            HookContext::BeforeVideoProcess(_) => HookPointV1::BeforeVideoProcess,
            HookContext::AfterPrepare(_) => HookPointV1::AfterPrepare,
            HookContext::OnCompleted(_) => HookPointV1::OnCompleted,
        }
    }

    pub fn to_input(&self, app_version: &str) -> eyre::Result<HookInputV1> {
        let hook_point = self.hook_point();
        let payload = match self {
            HookContext::BeforeVideoProcess(context) => {
                HookPayloadV1::BeforeVideoProcess(context.to_payload()?)
            }
            HookContext::AfterPrepare(context) => {
                HookPayloadV1::AfterPrepare(context.to_payload()?)
            }
            HookContext::OnCompleted(context) => HookPayloadV1::OnCompleted(context.to_payload()?),
        };

        let input = HookInputV1 {
            hook_point,
            payload,
            readonly_meta: HookReadonlyMetaV1 {
                app_version: app_version.to_string(),
                os: std::env::consts::OS.to_string(),
                arch: std::env::consts::ARCH.to_string(),
                process_id: std::process::id(),
            },
        };

        Ok(input)
    }

    pub fn apply_output(&mut self, output: HookOutputV1) -> eyre::Result<()> {
        let context_hook_point = self.hook_point();
        match (self, output.payload) {
            (
                HookContext::BeforeVideoProcess(context),
                HookPayloadV1::BeforeVideoProcess(payload),
            ) => context.apply_payload(payload),

            (HookContext::AfterPrepare(context), HookPayloadV1::AfterPrepare(payload)) => {
                context.apply_payload(payload)
            }

            (HookContext::OnCompleted(context), HookPayloadV1::OnCompleted(payload)) => {
                context.apply_payload(payload)
            }

            (_, payload) => Err(eyre!(
                "hook_point 与 payload 不匹配: hook_point={context_hook_point:?}, payload={payload:?}"
            )),
        }
    }
}

fn validate_task_id_unchanged(
    current_progress: &DownloadProgress,
    next_progress: &DownloadProgressV1,
) -> eyre::Result<()> {
    if current_progress.task_id != next_progress.task_id {
        return Err(eyre!("task_id 不可修改"));
    }
    Ok(())
}

fn host_to_api_progress(progress: &DownloadProgress) -> eyre::Result<DownloadProgressV1> {
    convert_via_json(
        progress,
        "序列化宿主 DownloadProgress 失败",
        "反序列化为插件 DownloadProgressV1 失败",
    )
}

fn api_to_host_progress(progress: DownloadProgressV1) -> eyre::Result<DownloadProgress> {
    convert_via_json(
        progress,
        "序列化插件 DownloadProgressV1 失败",
        "反序列化为宿主 DownloadProgress 失败",
    )
}

fn convert_via_json<TSrc, TDst>(
    source: TSrc,
    serialize_err: &str,
    deserialize_err: &str,
) -> eyre::Result<TDst>
where
    TSrc: Serialize,
    TDst: DeserializeOwned,
{
    let value = serde_json::to_value(source).wrap_err_with(|| serialize_err.to_string())?;
    serde_json::from_value(value).wrap_err_with(|| deserialize_err.to_string())
}
