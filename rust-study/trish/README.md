# trish

Rust 学习型 workspace，包含 14 个成员 crate：8 个命令行工具（其中两个是
clap derive / builder API 的对照教学项目）、3 个 ratatui TUI 程序、
2 个网络服务程序和 1 个工具库。所有 crate 共享 edition 2024、
workspace 级依赖与 lints。

## 项目一览

| Crate | 类型 | 说明 |
| --- | --- | --- |
| [minigrep](minigrep/) | CLI | 文本搜索工具（大小写、整词匹配、高亮输出） |
| [rcli](rcli/) | CLI | 通用工具（base64 编解码、随机密码、字数统计、基准测试） |
| [dante](dante/) | CLI | 多命令工具（greet / calc / echo / now） |
| [find_file](find_file/) | CLI | 文件查找（按扩展名、大小、修改时间、空文件） |
| [guess](guess/) | CLI | 多用途工具（哈希、HTTP 抓取、随机数、系统信息） |
| [mini](mini/) | CLI | 字符串 / 路径 / JSON 实用命令 |
| [basic](basic/) | CLI | clap builder API 教学（pacman 风格演示）+ Rust 基础演示 |
| [study](study/) | CLI | 文件操作工具（md5 / size / ls / add） |
| [counter](counter/) | TUI | ratatui 计数器（j/k 加减、数字键设步长） |
| [seele](seele/) | TUI | ratatui 交互演示（进度条 + 列表选择） |
| [ruanmei](ruanmei/) | TUI | 结构化 ratatui 应用模板（app / event / handler / ui） |
| [mini_web](mini_web/) | 服务 | mini-redis 客户端演示（需要本地 6379 端口的 redis） |
| [web_app](web_app/) | 服务 | axum Web 应用（健康检查、用户、产品路由） |
| [fade](fade/) | 库 | 字符串 / 数学工具函数库 |

所有 CLI 均支持 `--help`、`--version`（部分带 `completions --shell` 子命令）。

## 构建

```bash
cargo build --workspace            # debug
cargo build --workspace --release  # 优化构建，产物在 target/release/
```

## 测试

```bash
cargo test --workspace
```

每个项目都有单元测试；CLI 项目另含 `assert_cmd` 集成测试；
web_app 使用 tower oneshot 对路由做端到端测试。

## 质量门槛

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## License

未另行声明；minigrep 保留上游 MIT/Apache 双许可（见其目录）。
