# find_file

文件查找工具：按扩展名、大小、修改时间、空文件/目录查找，以及文件内容搜索。

## 用法

```bash
find_file png src/               # 递归查找 png
find_file txt docs/
find_file large . 1048576        # 大于 1MB 的文件
find_file recent src 7           # 最近 7 天修改的文件
find_file empty .                # 空文件与空目录
find_file query "TODO" src/main.rs   # 在文件中搜索包含该文本的行
find_file hostname               # 本机主机名
find_file completions --shell bash
```

## 命令

| 命令 | 说明 |
| --- | --- |
| `png <PATH>` / `txt <PATH>` | 按扩展名递归查找 |
| `large <PATH> <SIZE>` | 大于指定字节数的文件 |
| `recent <PATH> <DAYS>` | 最近 N 天修改的文件 |
| `empty <PATH>` | 空文件与空目录 |
| `query <QUERY> <FILE>` | 文件内容行搜索 |
| `hostname` | 主机名 |
| `completions --shell <SHELL>` | 生成 shell 补全 |

路径中的 Windows 反斜杠会被自动规范化（glob 使用 `/` 分隔）。

## 退出码

- `0` 成功
- `1` 运行错误（参数不是数字、文件读取失败）

## 开发

```bash
cargo test -p find_file
```
