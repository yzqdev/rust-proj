# web_app

axum Web 应用：控制器按模块划分，路由在 `build_router()` 组装。

## 用法

```bash
cargo run -p web_app
# 监听 0.0.0.0:3000
```

## 路由

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| GET | `/` | Hello, World! |
| GET / POST | `/foo` | 演示路由 |
| GET / POST | `/user` | 用户路由（POST 返回 201） |
| GET / POST / PUT / DELETE | `/api/products[/{id}]` | 产品 CRUD（内存存储） |
| GET | `/api/products/search?name=` | 产品搜索 |
| GET | `/health` | 健康检查（版本、运行时长） |
| GET | `/api/version` | 版本信息 |

## 测试

使用 `tower::ServiceExt::oneshot` 对路由做端到端测试（无需真实监听端口）：

```bash
cargo test -p web_app
```

日志级别通过 `RUST_LOG` 控制（默认 `web_app=debug`）。
