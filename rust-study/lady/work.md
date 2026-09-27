# 任务：完善当前项目中的 Rust CLI 子项目

当前项目中已经存在多个 Rust 子项目。请你全面检查这些 Rust 子项目，并在**保留现有功能和项目定位**的基础上，将它们完善成结构合理、功能完整、可以实际使用和发布的 CLI 项目。

## 一、总体目标

目标不是简单修复编译错误，而是：

> 把当前已有的 Rust 子项目，完善成具有真实 CLI 项目形态的 Rust 工程。

每个子项目都应该具备：

* 清晰的 CLI 入口
* 合理的命令结构
* 参数与选项
* 配置管理
* 错误处理
* 日志/输出
* 模块化代码结构
* 单元测试
* 集成测试或 CLI Smoke Test
* README 使用文档
* Cargo 元数据
* 格式化与静态检查
* Release 构建能力

不要为了“看起来完整”而无意义堆功能。优先围绕当前项目已有功能进行扩展。

---

# 二、第一阶段：全面分析现有 Rust 子项目

先不要立即修改代码。

遍历当前项目，找出所有 Rust 子项目，并分析：

* Cargo.toml
* Cargo.lock
* src/main.rs
* src/lib.rs
* src/bin/
* src/
* tests/
* examples/
* benches/
* README
* build.rs
* workspace 配置
* features
* dependencies
* 当前 CLI 参数
* 当前已有功能

同时检查：

1. 哪些项目是真正的 CLI。
2. 哪些项目只是 Rust 示例程序。
3. 哪些项目功能不完整。
4. 哪些项目存在重复代码。
5. 哪些项目可以抽取公共库。
6. 哪些项目应该拆分 `lib` 与 `bin`。
7. 哪些项目的依赖明显不合理。
8. 哪些项目存在过度设计。
9. 哪些项目缺少错误处理。
10. 哪些项目缺少测试。

先形成整体改造方案，再开始修改。

---

# 三、统一 Rust 工程规范

如果当前项目已经存在 Rust workspace，优先利用现有 workspace。

如果适合，可以整理成类似：

```text
rust-project/
├── Cargo.toml
├── crates/
│   ├── cli-a/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── cli.rs
│   │   │   ├── commands/
│   │   │   ├── config.rs
│   │   │   ├── error.rs
│   │   │   └── ...
│   │   └── tests/
│   │
│   ├── cli-b/
│   └── cli-c/
│
├── examples/
├── docs/
└── README.md
```

但不要为了套模板而强行移动项目。

如果某个项目规模很小，则可以保持：

```text
src/
├── main.rs
├── cli.rs
├── config.rs
├── error.rs
└── commands/
```

根据实际复杂度决定结构。

---

# 四、CLI 设计

优先使用 Rust 生态成熟的 CLI 库，例如：

* `clap`
* `clap_complete`
* `clap_mangen`

如果项目已经使用其他 CLI 库，先评估是否值得迁移，不要机械替换。

CLI 应该具备合理的：

### 全局参数

例如：

```text
--verbose
--quiet
--config
--output
--format
--no-color
--version
--help
```

只添加真正有意义的参数。

### 子命令

根据项目实际功能设计，例如：

```text
tool
├── init
├── config
├── list
├── get
├── add
├── remove
├── run
├── check
├── clean
└── version
```

不要强行加入不符合项目定位的命令。

### 帮助信息

确保：

```bash
tool --help
tool <command> --help
tool --version
```

输出清晰。

帮助信息应该能够让第一次使用项目的人理解：

* 工具是什么
* 每个命令干什么
* 参数怎么使用
* 常见示例是什么

---

# 五、配置系统

如果项目确实需要配置，请设计合理的配置体系。

例如：

```text
CLI 参数
    ↓
环境变量
    ↓
配置文件
    ↓
默认值
```

明确优先级。

可以根据实际需求选择：

* TOML
* JSON
* YAML

Rust 生态中优先考虑：

* `serde`
* `toml`
* `serde_json`

配置文件路径应考虑：

* 当前目录
* 用户配置目录
* `--config`
* 环境变量

不要为了配置而配置。

如果项目本身不需要持久化配置，则不要强行加入配置系统。

---

# 六、错误处理

禁止大量使用：

```rust
unwrap()
expect()
panic!()
```

在正常业务路径中。

根据项目规模选择：

* `thiserror`
* `anyhow`

建议形成类似：

```rust
type Result<T> = std::result::Result<T, Error>;
```

错误信息必须对 CLI 用户友好。

例如不要只输出：

```text
No such file or directory
```

而应该尽可能提供：

```text
Failed to read configuration file:
  path: ./config.toml

Reason:
  No such file or directory
```

同时保留错误 source，方便调试。

---

# 七、日志与终端输出

根据项目实际需求选择：

* `tracing`
* `tracing-subscriber`
* `env_logger`
* `console`
* `indicatif`
* `colored` / `owo-colors`

不要无脑引入全部依赖。

区分：

### 正常用户输出

例如：

```text
✓ Configuration loaded
✓ 12 items found
```

### 错误

输出到 stderr。

### Debug 日志

通过：

```text
-v
-vv
RUST_LOG
```

等方式控制。

确保：

```bash
tool command > output.txt
```

时，正常输出和错误/日志不会全部混在 stdout。

---

# 八、输出格式

如果 CLI 涉及列表、查询、数据处理等功能，考虑支持：

```text
--format table
--format json
--format jsonl
--format plain
```

但只有在项目确实存在结构化输出需求时才实现。

JSON 输出应该方便：

```bash
tool list --format json | jq
```

这样的命令行工作流。

---

# 九、模块化

避免把所有逻辑写在：

```rust
main.rs
```

里面。

推荐：

```text
main.rs
    ↓
cli.rs
    ↓
commands/
    ↓
service / domain
    ↓
filesystem / network / external API
```

CLI 层负责：

* 参数解析
* 调用业务逻辑
* 输出结果
* 将错误转换成 CLI 输出

业务层不要严重依赖 CLI。

如果某个功能值得复用，可以：

```text
src/lib.rs
```

提供公共 API。

然后：

```text
src/main.rs
```

只作为 CLI 入口。

---

# 十、异步支持

如果项目涉及：

* HTTP
* 网络请求
* 并发
* 异步 IO

合理使用：

```text
tokio
reqwest
```

但如果项目完全不需要异步，不要为了“现代 Rust 项目”强行加入 Tokio。

---

# 十一、文件系统与跨平台

如果 CLI 涉及文件操作：

优先使用：

```text
std::path::Path
std::path::PathBuf
```

而不是手工拼接字符串。

注意：

* Windows
* Linux
* macOS

路径差异。

如果涉及用户目录，考虑使用：

```text
directories
```

或 Rust 生态中合理的替代方案。

---

# 十二、测试

这是本次完善的重要部分。

至少补充：

## 单元测试

测试：

* 参数解析
* 配置解析
* 数据转换
* 核心业务逻辑
* 错误处理

例如：

```rust
#[test]
fn parse_config() {
    ...
}
```

## CLI 集成测试

推荐使用：

```text
assert_cmd
predicates
```

测试：

```bash
tool --help
tool --version
tool <command>
tool invalid-command
```

验证：

* exit code
* stdout
* stderr

## Smoke Test

确保项目构建完成后可以真正运行。

至少验证：

```bash
cargo check
cargo test
cargo clippy
cargo fmt --check
cargo build
```

如果项目适合，再验证：

```bash
cargo run -- --help
cargo run -- --version
```

---

# 十三、Cargo.toml 完善

检查每个项目的：

```toml
[package]
name = "..."
version = "..."
edition = "..."
description = "..."
license = "..."
repository = "..."
homepage = "..."
documentation = "..."
keywords = [...]
categories = [...]
```

根据项目实际情况补充。

不要虚构：

* license
* repository
* homepage
* 作者
* 联系方式

如果仓库信息未知，应保留现有信息或明确标记，而不是编造。

合理控制 dependencies。

删除：

* 未使用依赖
* 重复依赖
* 可以使用标准库替代的依赖
* 仅为了演示而存在但当前项目不需要的依赖

---

# 十四、Features

如果项目存在可选功能，可以合理使用：

```toml
[features]
default = [...]
foo = [...]
bar = [...]
```

例如：

```bash
cargo build --features foo
```

但不要为了展示 Cargo features 而人为制造复杂度。

---

# 十五、Release 构建

确保项目可以：

```bash
cargo build --release
```

并检查：

```text
target/release/
```

中的最终 CLI。

如果适合，可以完善：

* release profile
* strip
* LTO
* panic strategy

但要基于实际需求决定，不要盲目追求二进制体积。

---

# 十六、Shell Completion

如果 CLI 使用 `clap`，并且项目规模适合，可以增加：

```text
bash
zsh
fish
PowerShell
elvish
```

completion 支持。

例如：

```bash
tool completions bash
tool completions powershell
```

让项目更接近真实 CLI 工具。

---

# 十七、README

每个 CLI 项目都应该有完整 README。

至少包含：

```markdown
# Project Name

项目简介。

## Features

主要功能。

## Installation

安装方式。

## Usage

基本使用。

## Commands

命令说明。

## Configuration

配置说明。

## Examples

真实使用示例。

## Development

开发环境。

## Testing

测试方法。

## Build

构建方法。

## License
```

README 中的命令必须与实际项目保持一致。

禁止写不存在的命令。

---

# 十八、Examples

如果某个项目适合，增加：

```text
examples/
```

提供真实使用场景。

例如：

```text
examples/
├── basic.rs
├── config.rs
└── advanced.rs
```

如果项目是纯 CLI，则优先使用 README + CLI integration tests，不需要为了形式强行增加 examples。

---

# 十九、代码质量

统一执行：

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo check --all-targets --all-features
```

修复：

* compiler warning
* clippy warning
* dead code
* unnecessary clone
* unnecessary allocation
* needless borrow
* inefficient collection
* error handling 问题

但不要为了消除 Clippy warning 而牺牲代码可读性。

---

# 二十、Workspace 级别完善

如果这些项目属于同一个 Cargo workspace，请检查是否可以统一：

```toml
[workspace]
members = [...]

[workspace.dependencies]
...

[workspace.lints.rust]
...

[workspace.lints.clippy]
...
```

合理共享：

* Rust edition
* common dependencies
* lints
* package metadata

减少多个子项目重复配置。

---

# 二十一、不要过度重构

这是一个非常重要的要求。

不要为了“架构漂亮”而：

* 重写全部代码
* 删除现有功能
* 大规模修改 API
* 无理由增加大量依赖
* 把简单项目复杂化
* 强行引入异步
* 强行拆分几十个模块
* 强行增加数据库
* 强行增加网络功能

改造应该遵循：

```text
保留现有功能
    ↓
修复明显问题
    ↓
改善结构
    ↓
补充 CLI 能力
    ↓
补充测试
    ↓
补充文档
    ↓
完善工程化
```

---

# 二十二、最终验收标准

所有 Rust 子项目处理完成后，逐个验证。

至少确保：

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --workspace
cargo build --workspace --release
```

如果不是 workspace，则对每个子项目分别执行对应命令。

CLI 至少能够正常：

```bash
<command> --help
<command> --version
```

核心命令必须可以实际运行。

---

# 二十三、最终输出报告

完成后生成一份项目改造报告，例如：

```text
docs/rust-cli-improvement.md
```

报告包含：

## 1. Rust 子项目列表

| Project | 类型  | 状态 |
| ------- | --- | -- |
| xxx     | CLI | 完成 |
| xxx     | CLI | 完成 |

## 2. 每个项目修改内容

说明：

* 修改了什么
* 为什么修改
* 新增了哪些功能
* 新增了哪些依赖
* 是否进行了结构调整

## 3. CLI 命令

列出：

```text
tool --help
tool init
tool list
tool ...
```

## 4. 测试结果

记录：

```text
cargo fmt       PASS
cargo check     PASS
cargo test      PASS
cargo clippy    PASS
cargo build     PASS
release build   PASS
```

如果存在失败项，必须明确记录：

```text
FAIL
原因：
当前状态：
下一步：
```

禁止隐藏失败。

---

# 二十四、执行顺序

严格按照下面顺序执行：

### Step 1

扫描所有 Rust 子项目。

### Step 2

分析当前代码和项目定位。

### Step 3

制定每个项目的改造方案。

### Step 4

完善 Cargo / workspace。

### Step 5

完善 CLI。

### Step 6

重构代码结构。

### Step 7

完善错误处理、配置、输出等基础设施。

### Step 8

补充测试。

### Step 9

完善 README 和 examples。

### Step 10

运行 fmt / check / test / clippy / build。

### Step 11

修复所有发现的问题。

### Step 12

再次执行完整验证。

### Step 13

生成最终改造报告。

---

# 最重要的原则

请把这些项目当成**真实准备长期维护和发布的 Rust CLI 工具**来完善，而不是简单的 Rust 语法示例。

重点关注：

```text
CLI 体验
代码结构
错误处理
可测试性
可维护性
Cargo 工程规范
跨平台
文档
Release
```

同时保留当前项目已有的学习价值。

如果某个 Rust 子项目本身就是用于学习某项 Rust 技术，不要删除其教学代码；应该在保持教学目的的情况下，把它整理成一个完整、可运行、可理解的示例 CLI。

在修改任何文件之前，先完成项目扫描和分析；修改过程中优先小步迭代，每完成一个子项目就进行一次编译和测试，避免最后才集中处理大量问题。
