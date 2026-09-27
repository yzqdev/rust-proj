# mini_web

mini-redis 客户端演示：连接本地 redis，执行 set/get/publish 等操作。

## 前置条件

需要本地 6379 端口运行兼容 redis 协议的服务（如
[mini-redis-server](https://github.com/tokio-rs/mini-redis)：

```bash
cargo install mini-redis
mini-redis-server
```

## 用法

```bash
cargo run -p mini_web
```

演示内容：set/get、多键读取、过期键模拟、publish 3 条消息。
连接失败时会给出明确的错误提示（退出码 1）。
