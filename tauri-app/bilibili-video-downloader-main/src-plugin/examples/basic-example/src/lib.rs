use bilibili_video_downloader_plugin_sdk::{
    AfterPreparePayloadV1, BeforeVideoProcessPayloadV1, HookInputV1, HookOutputV1, HookPayloadV1,
    HookPointV1, OnCompletedPayloadV1, PluginDescriptorV1, PluginFailurePolicy, PluginV1,
    SDK_API_VERSION, export_plugin_v1,
    eyre::{self, eyre},
    host,
};

#[derive(Default)]
struct BasicExamplePlugin;

impl PluginV1 for BasicExamplePlugin {
    fn descriptor(&self) -> PluginDescriptorV1 {
        PluginDescriptorV1 {
            sdk_api_version: SDK_API_VERSION,
            id: "basic-example".to_string(),
            name: "Basic Example".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            hooks: vec![HookPointV1::BeforeVideoProcess, HookPointV1::AfterPrepare],
            failure_policy: PluginFailurePolicy::FailOpen,
            description: "基础示例插件：演示推荐的代码结构、在 Hook 中执行异步任务、读取宿主配置、处理 HookPayload，并修改 DownloadProgress".to_string(),
        }
    }

    fn on_hook(&mut self, input: HookInputV1) -> eyre::Result<HookOutputV1> {
        // 如果你需要用到异步，可以这样在同步 Hook 入口中创建 Tokio 运行时
        // 然后让代码在异步运行时里执行
        let output = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?
            .block_on(async { main(input).await })?;

        Ok(output)
    }
}

// 这并不是一个真的main函数，叫main只是为了方便理解
// 推荐将主要逻辑集中在此函数中，减少 on_hook 内部的嵌套层级
async fn main(input: HookInputV1) -> eyre::Result<HookOutputV1> {
    // 示例：读取宿主当前配置。
    let host_config = host::get_config()?;
    println!("{}", host_config.dir_fmt);

    // 示例：发起一次 HTTP 请求。
    let client = reqwest::Client::new();
    let body = client
        .get("https://jsonplaceholder.typicode.com/todos/1")
        .send()
        .await?
        .text()
        .await?;

    println!("HTTP 请求结果：{body}");

    let payload = match input.payload {
        HookPayloadV1::BeforeVideoProcess(payload) => handle_before_video_process(payload),
        HookPayloadV1::AfterPrepare(payload) => handle_after_prepare(payload),
        HookPayloadV1::OnCompleted(payload) => handle_on_completed(payload),
    }?;

    // 插件需要返回 payload，宿主会根据该返回值回写并更新自身状态
    // 所以插件内对 payload 的改动不会实时生效，只有当前 Hook 返回后，宿主才会应用这些修改
    Ok(HookOutputV1 { payload })
}

#[allow(clippy::unnecessary_wraps)]
fn handle_before_video_process(
    mut payload: BeforeVideoProcessPayloadV1,
) -> eyre::Result<HookPayloadV1> {
    println!("===========================BeforeVideoProcess========================");
    payload.progress.episode_title = "BeforeVideoProcess 修改了标题".to_string();
    Ok(HookPayloadV1::BeforeVideoProcess(
        BeforeVideoProcessPayloadV1 {
            progress: payload.progress,
        },
    ))
}

#[allow(clippy::unnecessary_wraps)]
fn handle_after_prepare(mut payload: AfterPreparePayloadV1) -> eyre::Result<HookPayloadV1> {
    println!("===========================AfterPrepare========================");
    payload.progress.episode_title = "AfterPrepare 修改了标题".to_string();
    Ok(HookPayloadV1::AfterPrepare(AfterPreparePayloadV1 {
        progress: payload.progress,
    }))
}

fn handle_on_completed(mut _payload: OnCompletedPayloadV1) -> eyre::Result<HookPayloadV1> {
    Err(eyre!(
        "插件未声明 OnCompleted HookPoint，按预期不应进入此分支。"
    ))
}

// 别把这行忘了
export_plugin_v1!(BasicExamplePlugin);
