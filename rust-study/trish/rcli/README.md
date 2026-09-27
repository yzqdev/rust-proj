# rcli

通用命令行工具：base64 编解码、随机密码生成、字数统计与简易基准测试。

## 安装

```bash
cargo install --path rcli
# 或在 workspace 根
cargo build --release -p rcli
```

## 用法

```bash
rcli encode "hello world"        # Encoded: aGVsbG8gd29ybGQ=
rcli decode aGVsbG8gd29ybGQ=     # Decoded: hello world
rcli genpwd --length 20 --special
rcli count --file Cargo.toml     # Lines: N, Words: N, Chars: N
rcli count "some text"           # 直接统计文本
rcli bench                       # 勾股数基准测试（耗时输出）
rcli completions --shell bash
```

全局参数：`[NAME]`（可选名称）、`-c/--config FILE`、`-d/-dd`（调试级别）。

## 退出码

- `0` 成功
- `1` 运行错误（base64 解码失败、文件读取失败等，错误输出到 stderr）

## 开发

```bash
cargo test -p rcli
cargo clippy -p rcli --all-targets -- -D warnings
```
