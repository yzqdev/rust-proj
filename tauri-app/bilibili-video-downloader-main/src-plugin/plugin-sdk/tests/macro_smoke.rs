use bilibili_video_downloader_plugin_sdk::{
    AfterPreparePayloadV1, BeforeVideoProcessPayloadV1, HookInputV1, HookOutputV1, HookPayloadV1,
    HookPointV1, OnCompletedPayloadV1, PluginDescriptorV1, PluginFailurePolicy, PluginV1,
    SDK_API_VERSION, export_plugin_v1, eyre, host,
};

#[derive(Default)]
struct MacroSmokePlugin;

impl PluginV1 for MacroSmokePlugin {
    fn descriptor(&self) -> PluginDescriptorV1 {
        PluginDescriptorV1 {
            sdk_api_version: SDK_API_VERSION,
            id: "macro-smoke".to_string(),
            name: "Macro Smoke".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            hooks: vec![HookPointV1::BeforeVideoProcess],
            failure_policy: PluginFailurePolicy::FailOpen,
            description: "Compile-time macro smoke test".to_string(),
        }
    }

    fn on_hook(&mut self, input: HookInputV1) -> eyre::Result<HookOutputV1> {
        let payload = match input.payload {
            HookPayloadV1::BeforeVideoProcess(payload) => {
                HookPayloadV1::BeforeVideoProcess(BeforeVideoProcessPayloadV1 {
                    progress: payload.progress,
                })
            }
            HookPayloadV1::AfterPrepare(payload) => {
                HookPayloadV1::AfterPrepare(AfterPreparePayloadV1 {
                    progress: payload.progress,
                })
            }
            HookPayloadV1::OnCompleted(payload) => {
                HookPayloadV1::OnCompleted(OnCompletedPayloadV1 {
                    progress: payload.progress,
                })
            }
        };

        Ok(HookOutputV1 { payload })
    }
}

export_plugin_v1!(MacroSmokePlugin);

#[test]
fn macro_smoke_builds() {
    assert_eq!(SDK_API_VERSION, 1);

    let err = host::get_config().unwrap_err();
    assert!(err.to_string().contains("host api 未注册"));
}
