# 插件系统（v1，实验性）

> [!WARNING]
> 插件是进程内动态库(`dll` / `so` / `dylib`)，与宿主进程同权限运行  
> 没有沙箱、权限隔离、签名校验，也没有网络或文件系统限制  
> 插件还可以读取完整的宿主配置(包括 `sessdata`)

## 当前示例

- 当前仓库的示例插件只有一个：`basic-example`
- 位置：`src-plugin/examples/basic-example`
- 用途：演示 Descriptor、Hook 处理、异步逻辑、读取宿主配置、返回修改后的 `payload`

## 快速构建示例

```bash
cd src-plugin/examples
cargo build --release
```

构建产物位于 `src-plugin/examples/target/release/`，文件名按平台分别是：

- Windows: `basic_example.dll`
- Linux: `libbasic_example.so`
- macOS: `libbasic_example.dylib`

## 如何加载插件

- 在应用设置页的插件面板点击“添加插件”，选择动态库文件
- 后端要求路径必须是绝对路径，且文件必须存在
- 成功后会写入 `app_data_dir/plugin.json` 持久化配置
- `plugin.json` 保存字段：`path`、`enabled`、`priority`、`descriptor`

## Descriptor（插件内声明）

`PluginDescriptorV1` 由插件代码返回，宿主不会直接修改：

- `sdk_api_version`：SDK API 版本，当前必须等于 `1`
- `id`：插件 ID
- `name`：展示名称
- `version`：插件版本
- `hooks`：声明插件希望被调用的 Hook 点列表
- `failure_policy`：失败策略，`FailOpen` 或 `FailClosed`
- `description`：插件描述

## 运行顺序与优先级

- Hook 点相同的插件，宿主按 `priority` 从大到小执行
- 每个 Hook 点按顺序串行执行，不是并行
- 前一个插件返回后的修改，会成为后一个插件看到的输入

## Hook 时机（以实际代码为准）

| HookPoint            | 触发位置                        |
|:---------------------|:----------------------------|
| `AfterPrepare`       | `prepare()` 成功之后，开始下载前      |
| `BeforeVideoProcess` | 视频任务和音频任务结束后，视频处理任务前        |
| `OnCompleted`        | 所有任务结束后，`completed_ts` 已写入后 |

三个 Hook 都可读写 `progress`，但修改只有在当前 Hook 返回后才会被宿主应用。

## 输入输出协议

- 输入：`HookInputV1 { hook_point, payload, readonly_meta }`
- 输出：`HookOutputV1 { payload }`
- `payload` 是枚举 `HookPayloadV1`，必须与 `hook_point` 匹配
- 不匹配会被判定为插件输出无效，再按失败策略处理
- `readonly_meta` 包含 `app_version`、`os`、`arch`、`process_id`

## 可修改范围与约束

- `payload.progress` 大多数字段都可被插件修改
- `task_id` 明确禁止修改，改动会被宿主拒绝
- 宿主没有提供修改Config的 Host API

## 失败策略

- `FailOpen`：插件出错时记录日志，继续执行后续流程
- `FailClosed`：插件出错时中断当前下载流程并返回错误
- 插件返回 `Err(...)` 时，宿主会调用 `bilibili_video_downloader_plugin_last_error_v1` 读取错误文本

## Host API（当前仅 v1）

插件可在 `on_hook` 中调用：

- `host::get_config()`：读取宿主配置快照（`HostConfigV1`）

注意：

- 该配置是只读快照
- 返回内容包含敏感信息（例如 `sessdata`）

## SDK 入口（插件侧）

- 实现 trait：`PluginV1`
- 使用宏导出：`export_plugin_v1!(YourPluginType)`
- 插件类型需要满足：`Default + Send + 'static`

常规最小结构：

```rust
use bilibili_video_downloader_plugin_sdk::{
    HookInputV1, HookOutputV1, PluginDescriptorV1, PluginV1, export_plugin_v1, eyre
};

#[derive(Default)]
struct MyPlugin;

impl PluginV1 for MyPlugin {
    fn descriptor(&self) -> PluginDescriptorV1 {
        unimplemented!()
    }

    fn on_hook(&mut self, input: HookInputV1) -> eyre::Result<HookOutputV1> {
        let _ = input;
        unimplemented!()
    }
}

export_plugin_v1!(MyPlugin);
```
