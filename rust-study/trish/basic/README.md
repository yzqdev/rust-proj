# basic

clap builder API 教学 CLI：pacman 风格包管理演示 + Rust 基础特性演示。

## 用法

```bash
basic query --search rust            # 本地包查询演示
basic sync -s rust                   # 远程搜索演示
basic sync --info crate-a crate-b    # 包信息演示
basic generic                        # 泛型演示
basic demo --feature struct          # struct | array | string | all
basic completions --shell bash
```

## 子命令

| 命令 | 说明 |
| --- | --- |
| `query` (`-Q`) | 包数据库查询演示 |
| `sync` (`-S`) | 包同步演示 |
| `generic` | 泛型演示 |
| `demo --feature <FEATURE>` | Rust 基础演示 |
| `completions --shell <SHELL>` | 生成 shell 补全 |

## 开发

```bash
cargo test -p basic
```

`src/datatype/`、`src/syntax/` 及各测试文件保留了所有权、借用、解构、
泛型等教学示例代码。
