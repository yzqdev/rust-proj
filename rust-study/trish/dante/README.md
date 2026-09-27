# dante

多命令 CLI 工具：问候、四则运算、回显、当前时间。

## 用法

```bash
dante greet Rust --count 2
dante calc 6 --op mul 7          # 6 mul 7 = 42
dante calc 1 --op div 0          # Error: division by zero（退出码 1）
dante echo --upper hello world
dante now --format date          # date | time | datetime | timestamp
dante completions --shell zsh
```

## 命令

| 命令 | 说明 |
| --- | --- |
| `greet <NAME> [-c COUNT]` | 问候指定次数 |
| `calc <A> --op <add|sub|mul|div> <B>` | 四则运算（也接受 + - * /） |
| `echo [TEXT]... [--upper]` | 回显（可转大写） |
| `now [--format ...]` | 当前日期/时间/时间戳 |
| `completions --shell <SHELL>` | 生成 shell 补全 |

## 退出码

- `0` 成功
- `1` 运行错误（除零、未知运算符）

## 开发

```bash
cargo test -p dante
```
