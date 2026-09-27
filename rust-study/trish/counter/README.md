# counter

ratatui 计数器 TUI 演示。

## 用法

```bash
cargo run -p counter
```

| 按键 | 功能 |
| --- | --- |
| `j` / `k` | 计数 +/- 步长 |
| `1-9` | 设置步长 |
| `r` | 归零 |
| `q` | 退出 |

状态与按键处理在 `src/lib.rs`（`State::on_char`），含单元测试；
终端在任何错误路径下都会恢复。
