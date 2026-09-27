# study

文件操作学习工具（clap builder API）。

## 用法

```bash
study md5 Cargo.toml        # MD5(Cargo.toml) = <hash>
study size poem.txt         # 人类可读大小
study ls src                # 目录列表（目录在前，含大小）
study add 2 3               # 2 + 3 = 5
study clone <REMOTE>        # 模拟 clone
study completions --shell bash
```

## 命令

| 命令 | 说明 |
| --- | --- |
| `md5 <FILE>` | 计算文件 MD5 |
| `size <FILE>` | 文件大小 |
| `ls <DIR>` | 列出目录内容 |
| `add <A> <B>` | 两数相加 |
| `clone <REMOTE>` | 模拟 clone |
| `completions --shell <SHELL>` | 生成 shell 补全 |

## 退出码

- `0` 成功
- `1` 运行错误（文件/目录不存在、参数不是数字）

## 开发

`src/macro_use/`、`src/trait_use/`、`src/io_use/conf_constant.rs` 保留
宏、trait、常量等教学示例。

```bash
cargo test -p study
```
