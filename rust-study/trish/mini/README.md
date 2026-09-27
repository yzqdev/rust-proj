# mini

字符串 / 路径 / JSON 实用命令工具。

## 用法

```bash
mini string reverse "hello"          # olleh
mini string count "a b c"            # Word count: 3
mini string snake "camelCaseText"    # camel_case_text
mini path ext a/b.txt                # Extension: txt
mini path parent a/b.txt             # Parent: a
mini json encode                     # json/cbor/bincode 序列化演示
mini gen                             # 文件 + JSON 综合演示
mini completions --shell bash
```

## 命令

| 命令 | 说明 |
| --- | --- |
| `string reverse <TEXT>` | 反转字符串 |
| `string count <TEXT>` | 单词计数 |
| `string snake <TEXT>` | 转换为 snake_case |
| `path ext <PATH>` | 取扩展名 |
| `path parent <PATH>` | 取父目录 |
| `json encode` | serde 多格式序列化演示 |
| `gen [NAME]` | 综合演示 |
| `completions --shell <SHELL>` | 生成 shell 补全 |

## 开发

```bash
cargo test -p mini
```
