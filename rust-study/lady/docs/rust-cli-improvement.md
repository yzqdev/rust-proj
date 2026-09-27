# Rust CLI 改造报告

日期：2026-09-28
范围：`lady` workspace 全部 8 个成员 crate（7 个 CLI + 1 个新增公共库）

## 1. Rust 子项目列表

| Project | 类型 | 状态 |
| --- | --- | --- |
| asta | CLI（哈希计算器） | 完成 |
| httpman | CLI（HTTP 客户端） | 完成 |
| taoqi | CLI（JSON 处理器） | 完成 |
| phoebe | CLI（CSV→JSON 转换 + 安装模拟） | 完成 |
| robin | CLI（文件工具） | 完成 |
| pela | CLI（教学：clap derive 版 git 模拟） | 完成 |
| sampo | CLI（教学：clap builder 版 git 模拟） | 完成 |
| hash-utils | 公共库（新增） | 完成 |

## 2. 每个项目修改内容

### Workspace 级别（根 Cargo.toml）

* 新增 `[workspace.package]`（edition 2024 / authors / version），成员以 `xxx.workspace = true` 继承。
* 新增 `[workspace.dependencies]` 统一管理 clap、clap_complete、anyhow、thiserror、serde_json、csv、colored、md-5、sha2、digest、jsonxf、mime、reqwest、tokio、assert_cmd、predicates。
* 新增 `[workspace.lints]`（`unsafe_code = "forbid"`、`clippy::all = "warn"`），全部成员启用 `[lints] workspace = true`。
* 新增 `[profile.release]`：`strip = true`、`lto = "thin"`。
* 新增根 README.md 描述 workspace 结构与每个工具。

### 新增公共库 hash-utils

* 原因：asta 与 robin 各自实现了几乎相同的"分块读取文件 + MD5"代码（robin 内部还有两份重复）。
* 提供 `Algorithm`、`Error`（thiserror）、`md5_hex` / `sha256_hex` / `file_hash`（8 KiB 分块流式读取）。
* 单元测试 5 个（含已知向量 MD5/SHA256("hello")、缺失文件错误、错误信息文案）。

### asta（哈希计算器）

* 拆分 `lib.rs`（CLI 定义 + `run() -> Result<String>`）与 `main.rs`（薄入口，`ExitCode`）。
* `file` 子命令新增 `--algorithm md5|sha256`（默认 md5，保持原行为），底层改用 hash-utils。
* `--version` 由硬编码 `"1.0"` 改为 Cargo 包版本（`#[command(version)]`）。
* 新增 `completions --shell` 子命令。
* 错误处理：`unwrap_or_else(exit)` → `thiserror` 错误 + 退出码 1。
* 删除未使用依赖 `hex-literal`；删除无意义占位测试 `test_chain.rs`（内容仅 `println!("hello world")`）。
* 新增 9 个测试（lib 单测 + assert_cmd 集成测试）。

### httpman（HTTP 客户端）

* 修复重大缺陷：`lib.rs` 原为空文件，`util.rs` 的教学宏实际未导出，`test_chain.rs` 等集成测试从未通过编译（`cargo check` 默认不检查 tests，掩盖了该问题）。现在 `lib.rs` 声明 `pub mod util;`，宏在 crate 根可用。
* 业务逻辑入 lib：`KvPair` 解析、`parse_url`、`build_request`。
* **修复 `KvPair::from_str`**：改用 `splitn(2, ...)`，值中含 `=`/`:` 不再被截断；空 key 报错；空值允许。
* `build_request` 中非法 header 由"eprintln 后静默丢弃"改为返回明确错误。
* main 错误输出统一为 `Error: {err:#}`（anyhow 链），退出码 1；新增 `completions` 子命令。
* 删除未使用依赖 `md-5`、`hex-literal`、`digest`。
* 保留全部教学代码（`util.rs` 宏、`test_display/test_var/test_io/test_panic`），仅做最小修复：`test_io.rs` 中会 panic 的不存在路径 `e:/tmpgit` → `./`；`test_display.rs` 中 `env::var("android_proj").unwrap()` 改为 `unwrap_or_else`；`test_panic.rs` 的故意 panic 拆分为 `#[should_panic]` 测试；文件头加 allow 属性保留教学写法。
* 新增 lib 单测 8 个 + CLI 集成测试 5 个（不发真实网络请求）。

### pela（clap derive 教学版 git 模拟）

* 拆分 lib/main；`init`/`clone`/`add` 落地为返回 `Result<String, Error>` 的函数。
* 消除 `File::create(...).unwrap()`、`to_str().unwrap()`；错误带上下文（"failed to create repository directory: ..."）。
* `add` 遇到不存在的路径：原"打印 stderr 但退出码 0"改为退出码 1 的错误。
* diff 参数由 OsString 简化为 String（消除 `to_str().unwrap()`）。
* 路径拼接改用 `PathBuf::join`（跨平台）。
* 新增 `completions` 子命令；删除未使用依赖 `md-5`、`hex-literal`、`digest`；删除占位测试。
* 新增 lib 单测 4 个 + 集成测试 8 个。

### sampo（clap builder 教学版 git 模拟）

* 与 pela 同一 CLI 的 builder API 版，二者作为对照教学保留。
* 业务函数入 lib（`init_repo`/`clone_repo`/`add_paths`/`commit`/... 返回 `Result<String, Error>`）。
* 消除 `expect("required")`/`unreachable!()`：统一 `required()` 辅助函数 + `Error::MissingArgument`。
* 新增 `completions` 子命令；删除未使用依赖 `chrono`。
* 新增 lib 单测 4 个 + 集成测试 6 个。

### phoebe（CSV→JSON + 安装模拟）

* CSV 转换逻辑入 lib（`csv_to_json<R: Read>`，内存 reader 即可测试）；删除空的 `src/cli.rs`。
* 修复 `--header` 标志缺陷：原 `default_value_t = true` 的 SetTrue action 导致**永远无法传 false**；改为 `num_args=0..=1 + require_equals + default_missing_value`，`--header=false` 可用且 `--header` 语义不变。
* 非 ASCII 分隔符（会静默截断为错误字节）现在显式报错。
* 错误信息友好化、退出码 1；删除重复打印的 `--param` 输出；删除未使用依赖 `serde`（保留 serde_json）。
* 新增 lib 单测 6 个 + 集成测试 7 个。

### robin（文件工具）

* 删除重复代码：`src/util.rs` 的 `gen_fsmd5` 与 `file_cmd::calc_md5` 重复，统一改用 hash-utils 后删除该文件。
* `cmd/file_cmd.rs` 四个函数全部改为返回 `Result<String, Error>`（thiserror：NotFound / NotADirectory / Io）。
* **修复退出码**：原所有错误只打印 stderr 但退出码为 0；现在 main 返回 `ExitCode`，错误时退出码 1。
* 错误经 `Error: ...` 统一输出；新增 `completions` 子命令。
* 删除未使用依赖 `hex-literal`；保留 `add --num` 演示命令（教学用途）。
* 新增 lib 单测 6 个（含 `format_bytes` 边界）+ 集成测试 7 个。

### taoqi（JSON 处理器）

* 拆分 lib/main：`load_json`、`query_value`、`root_type` + `run()`。
* 错误枚举（thiserror）覆盖：IO 读取/写入、非法 JSON（保留 ❌ 前缀的原始输出语义）、key 缺失、非数字索引、越界、对标量取索引——全部带上下文，替代原来的 `eprintln + exit(1)` 散落写法。
* 消除 `to_string_pretty(...).unwrap()`（解析后的 Value 序列化不可能失败，改用 `expect` 注释说明——serde 内部不变量，非业务路径）。
* `file` 参数由 String 改为 PathBuf；新增 `completions` 子命令。
* 新增 lib 单测 4 个 + 集成测试 10 个。

## 3. CLI 命令一览

```text
asta      md5 <TEXT> | sha256 <TEXT> | file <PATH> [--algorithm md5|sha256] | completions --shell
httpman   get|post|put|delete|head|patch <URL> [PAIRS...] | completions --shell
taoqi     format <FILE> [--in-place] | validate <FILE> | minify <FILE> [--in-place]
          | query <FILE> <PATH> | completions --shell
phoebe    csv [-i IN] [-o OUT] [-d CHAR] [--header=BOOL] [-p] | install [NAME] [--latest --param -g] | completions --shell
robin     md5 <FILE> | info <FILE> | img <FILE> | tree <DIR> | add --num <N> | completions --shell
pela      init|clone|diff|push|add|commit|status|stash [push|pop|apply|list]|log|completions + 外部子命令透传
sampo     同 pela（builder API 实现）
```

每个 CLI 的 `--help` / `--version` / `completions --shell <SHELL>` 均已实测。

## 4. 测试结果

```text
cargo fmt --all -- --check                                PASS
cargo check --workspace --all-targets --all-features      PASS
cargo test  --workspace                                   PASS (105 passed, 0 failed)
cargo clippy --workspace --all-targets --all-features
               -- -D warnings                             PASS (0 warning)
cargo build --workspace                                   PASS
cargo build --workspace --release                         PASS (7 个二进制已生成)
```

实测（target/release）：

```text
asta sha256 hello          -> 2cf24dba...9824（与已知向量一致）
taoqi query tq.json b.c.1  -> 20
taoqi minify tq.json       -> {"a":1,"b":{"c":[10,20]}}
robin md5 h.txt            -> b1946ac92492d2347c6235b4d2611184（MD5("hello\n")）
phoebe install demo-tool   -> ./node_modules/demo-tool.json
sampo init tq-proj         -> .tq-proj/ 已创建
所有 7 个二进制 --version / --help 正常
```

无失败项。

## 5. 依赖变化汇总

| 变更 | 内容 |
| --- | --- |
| 删除（未使用） | `hex-literal`（asta/httpman/robin）、`md-5`/`digest`（httpman、pela）、`chrono`（sampo）、`serde`（phoebe，仅用 serde_json） |
| 新增 | `clap_complete`（全部 CLI，completions）、`thiserror`（结构化错误）、`hash-utils`（asta/robin）、dev-deps `assert_cmd`/`predicates`（全部 CLI） |
| 收敛 | 全部依赖提升为 `[workspace.dependencies]` 统一版本 |

## 6. 结构调整汇总

* 全部 7 个 CLI 统一为 `lib.rs`（CLI 定义 + 业务逻辑 + 单测）/ `main.rs`（薄入口）/ `tests/cli.rs`（集成测试）三层结构。
* 新增 `hash-utils` 公共库消除 3 处重复哈希实现。
* 删除：phoebe 空模块 `cli.rs`、robin 重复的 `util.rs`、3 个无意义占位 `test_chain.rs`。
* 教学代码全部保留：httpman 宏与 RBE 风格教学测试、pela/sampo 的 derive/builder 对照实现、robin 的 `add` 演示命令。
