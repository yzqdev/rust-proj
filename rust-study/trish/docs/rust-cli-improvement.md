# trish workspace Rust 项目改造报告

日期：2026-09-28
范围：`trish` workspace 全部 14 个成员 crate

## 1. 项目列表与定位

| Project | 类型 | 状态 |
| --- | --- | --- |
| minigrep | CLI（文本搜索） | 完成（轻量完善） |
| rcli | CLI（base64/密码/统计） | 完成 |
| dante | CLI（greet/calc/echo/now） | 完成 |
| find_file | CLI（文件查找） | 完成 |
| guess | CLI（哈希/抓取/随机数） | 完成 |
| mini | CLI（字符串/路径/JSON） | 完成 |
| basic | CLI（clap builder 教学 + 基础演示） | 完成 |
| study | CLI（文件操作） | 完成 |
| counter | TUI（ratatui 计数器） | 完成 |
| seele | TUI（ratatui 交互演示） | 完成 |
| ruanmei | TUI（结构化模板） | 完成（轻量完善） |
| mini_web | 服务（mini-redis 客户端演示） | 完成 |
| web_app | 服务（axum Web 应用） | 完成 |
| fade | 库（字符串/数学工具） | 完成（轻量完善） |

## 2. 修改内容

### Workspace 级别（根 Cargo.toml）

* 新增 `[workspace.package]`（edition 2024 / version / authors）与 `[workspace.dependencies]`，
  统一 clap 4.5.30、tokio（features 收敛为 rt/rt-multi-thread/macros）等 18 个依赖。
* 新增 `[workspace.lints]`（`unsafe_code = "forbid"`、`clippy::all = "warn"`），全部成员启用。
* 新增 `[profile.release]`：`strip = true`、`lto = "thin"`。
* 保留 minigrep 的上游作者与 LICENSE 文件、ruanmei 的 MIT license 声明（未虚构元数据）。

### 发现并修复的原始 Bug

| 项目 | Bug | 修复 |
| --- | --- | --- |
| guess | `random` 子命令 `min`/`max` 均声明 `-m` 短选项，clap debug assert 直接 panic —— **`guess random` 从未可用过** | 移除重复短选项，仅保留 `--min`/`--max` |
| mini | `mini gen <x>` 内部 `get_many("gen").unwrap()` 取错参数 id，必然 panic | 改为使用实际定义的参数；无子命令时 `todo!()` panic 改为 `arg_required_else_help` |
| web_app | `user_route` 中 `route("/user", post(create_user));` 返回值被丢弃，POST /user 从未注册 | 在同一 Router 上同时注册 GET handler 与 POST create_user |
| find_file | glob pattern 用原始路径拼接，Windows 反斜杠被 glob 当作转义符 | `build_pattern()` 统一把 `\` 规范化为 `/`（带测试） |
| study | `src/main.rs` 与 `lib.rs` 重复声明同一批模块（两份编译产物） | main 仅引用 lib |
| guess | `command_test.rs` 依赖本地服务（127.0.0.1:8550），改造前测试套件即失败 | 标记 `#[ignore]`（本地起服务后可 `--ignored` 手动运行） |
| dante | 除零/未知运算符只打印 stderr 但退出码 0 | 返回错误，退出码 1 |

### 各项目要点

* **rcli**：base64 编解码、密码生成、字数统计全部入 lib 并可单测
  （base64 用 RFC 已知向量测试）；新增 `completions` 子命令；错误经
  thiserror 输出、退出码 1。
* **dante**：逻辑入 lib（`calc()` 返回 `Result`）；新增 `completions`；
  lib 单测 6 个 + 集成测试 7 个。
* **mini**：新增 `string snake` 子命令（利用已有的 `to_snake_case`）；
  新增 `completions`；serde_derive 合并进 serde derive feature；
  lib 单测覆盖 string/path 工具。
* **basic**：`parse_number`/`sum_array`/`capitalize` 等纯函数入 lib；
  保留 pacman 风格演示与 `demo` 子命令；硬编码版本号改为 Cargo 版本；
  `unreachable!()` 替换为显式处理；教学模块加最小 allow。
* **study**：`file_size`/`list_dir` 返回 `Result<String>`；删除读取
  命令行参数的死代码 `get_file_md5`；`add` 参数解析失败不再 panic；
  删除依赖 reqwest/tokio/dirs（未使用）与依赖网络的教学测试
  `req_test.rs`（内容与 guess fetch 重复）；新增 `completions`。
* **guess**：CLI 操作入 lib（`core_ops`：hash/read/info/fetch/random），
  fetch 经 reqwest 错误链友好输出；`random` 对反序 min/max 安全处理；
  教学模块（simple/command/core/util）标注文档并隔离 panic 代码；
  新增 `completions`。
* **find_file**：glob 查找逻辑入 lib（5 个带临时目录测试的函数）；
  把遗留的"hostname 读硬编码 Windows 路径"改为环境变量查主机名；
  遗留死代码 `main_fun`/`Config` 替换为真正可用的 `query` 内容搜索
  子命令（复用原 `search()` 函数）；新增 `completions`。
* **minigrep**（上游项目，轻量完善）：保留上游结构；clippy 修复
  （match→if let、字段简写、`Ok(?)` 冗余）；README 更新为实际 CLI
  （原 README 与实际参数不一致）；新增 assert_cmd 集成测试 4 个。
* **counter / seele**：状态与按键处理抽取到 `src/lib.rs`
  （`State::on_char` / `Action`），新增单元测试；终端在任何错误路径
  （包括 draw/poll 失败）下都会恢复 raw mode / alternate screen。
* **ruanmei**：修复 ratatui 0.29 弃用 API（`frame.size()` → `frame.area()`）、
  未用导入；新增 `tests/app_test.rs`（5 个状态机测试）。
* **mini_web**：删除未使用依赖 axum、clap（实际是 mini-redis 客户端
  演示，不是 Web 服务）；连接失败输出明确提示与前置条件（退出码 1）；
  README 说明需要先启动 redis 服务。
* **web_app**：路由组装入 lib（`build_router()`）；修复 POST /user 注册
  丢失；`print_hello` 构建期副作用移除；bind 失败退出码 1；新增 tower
  oneshot 端到端测试 5 个（/、/foo GET+POST、/user POST、/health、/api/products）。
* **fade**：`is_prime` 使用 `is_multiple_of`（clippy 建议）；补充 README。

### 依赖变化

| 变更 | 内容 |
| --- | --- |
| 删除（未使用） | `axum`、`clap`（mini_web）；`reqwest`、`tokio`、`dirs`（study） |
| 合并 | `serde_derive` → `serde` derive feature（mini） |
| 新增 | `clap_complete`（8 个 CLI）、`thiserror`（rcli/dante/find_file/guess）、dev-deps `assert_cmd`/`predicates`（6 个 CLI）、dev-deps `tower`/`http-body-util`（web_app） |
| 收敛 | 全部依赖提升为 `[workspace.dependencies]` |

### 测试结果

```text
cargo fmt --all -- --check                                PASS
cargo check --workspace --all-targets --all-features      PASS
cargo clippy --workspace --all-targets --all-features
               -- -D warnings                             PASS (0 error)
cargo test  --workspace                                   PASS (108 passed, 0 failed)
cargo build --workspace                                   PASS
cargo build --workspace --release                         PASS (14 个二进制)
```

实测（target/release）：`dante calc 6 --op mul 7` → `6 mul 7 = 42`；
`rcli encode "hello world"` → `aGVsbG8gd29ybGQ=`；`study md5`、
`guess hash`（与已知向量一致）；`find_file png`、`minigrep --count`、
`mini string snake` 均正常；8 个 CLI 的 `--help`/`--version` 正常。

### 已知限制（如实记录）

* `guess tests/command_test.rs::req` 标记 `#[ignore]`：需要本地 8550
  端口服务。原状为直接失败。
* `mini_web` 需要本地 redis（6379）才能运行成功，README 已注明。
* minigrep 保留了上游作者的原始代码风格（含 `assert_eq!(x, true)` 断言、
  旧式 error 结构体），仅做 clippy 修复与文档对齐，未重写。
