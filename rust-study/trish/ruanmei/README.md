# ruanmei

结构化 ratatui 应用模板（rust-ratatui-template 风格）：
app 状态 / event 事件循环 / handler 按键处理 / ui 渲染 / tui 终端管理。

## 用法

```bash
cargo run -p ruanmei
```

| 按键 | 功能 |
| --- | --- |
| `←` / `→` | 计数 -/+ |
| `↑` / `↓` | 列表选择 |
| `r` | 计数归零 |
| `q` / `Esc` / `Ctrl-C` | 退出 |

`tests/app_test.rs` 对应用状态与按键处理做了单元测试。

## License

MIT
