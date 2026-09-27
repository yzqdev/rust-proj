# guess

多用途 CLI 工具：MD5、HTTP 抓取、随机数、文件读取、系统信息。

## 用法

```bash
guess hash "hello"                # MD5("hello") = 5d4140...
guess fetch https://httpbin.org/get
guess random --min 5 --max 10
guess read poem.txt
guess info                        # OS / Arch / 当前目录
guess core                        # 教学演示
guess completions --shell bash
```

## 命令

| 命令 | 说明 |
| --- | --- |
| `hash <TEXT>` | 字符串 MD5 |
| `fetch <URL>` | 抓取 URL，输出状态与正文预览（前 500 字符） |
| `random [--min N] [--max N]` | 闭区间随机数 |
| `read <FILE>` | 读取并显示文本文件 |
| `info` | 系统信息 |
| `core` | 教学演示 |
| `completions --shell <SHELL>` | 生成 shell 补全 |

## 退出码

- `0` 成功
- `1` 运行错误（文件不存在、请求失败、URL 非法）

## 开发

`src/simple.rs`、`src/command/`、`src/core/`、`src/util/` 保留教学示例。
注意 `random` 对反序的 min/max 做了安全处理（返回 min 而非 panic）。

```bash
cargo test -p guess
```
