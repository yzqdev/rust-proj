# rust-web

一个结构完整、可直接运行的 Rust Web API 示例：**任务管理（Task Board）**。

技术栈：**Axum 0.8** + **SQLx 0.9（SQLite）** + **Tokio** + **tracing**。

## 特性

- 分层清晰：`routes`（HTTP）→ `repo`（SQL）→ `models`（领域模型），换库只需改 `repo`
- 统一错误类型 `AppError`，任何地方都能用 `?`，最终输出固定形状的 JSON 错误
- 请求校验、`PATCH` 局部更新（`null` 与「不传」语义分明）
- SQLite 迁移随二进制内嵌（`sqlx::migrate!`），启动自动执行
- 结构化日志（`tracing`）+ 请求追踪（`TraceLayer`）+ CORS（`CorsLayer`）
- 优雅关闭（Ctrl-C / SIGTERM）
- 自带一个单文件前端页面，打开浏览器即可操作全部接口
- 8 个集成测试，不监听端口、不需要外部数据库

## 快速开始

```bash
# 1. 可选：准备配置（不影响运行，全部有默认值）
cp .env.example .env

# 2. 启动
cargo run
```

服务默认监听 `http://127.0.0.1:8080`：

| 地址 | 说明 |
| --- | --- |
| <http://127.0.0.1:8080/> | 任务看板页面 |
| <http://127.0.0.1:8080/health> | 存活探针 |
| <http://127.0.0.1:8080/about> | 服务信息与端点清单 |

数据库文件首次启动时自动创建在 `data/app.db`。

```bash
cargo test      # 集成测试
cargo fmt       # 格式化
cargo clippy --all-targets -- -D warnings
```

## 配置

全部来自环境变量（`.env` 会被自动加载）：

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `APP_HOST` | `127.0.0.1` | 监听地址 |
| `APP_PORT` | `8080` | 监听端口 |
| `DATABASE_URL` | `sqlite://data/app.db` | SQLx 连接串 |
| `DB_MAX_CONNECTIONS` | `5` | 连接池大小 |
| `STATIC_DIR` | `static` | 静态目录；置空则不提供页面 |
| `CORS_PERMISSIVE` | `true` | 是否允许任意跨域来源 |
| `RUST_LOG` | `info,rust_web=debug,tower_http=debug` | 日志过滤 |

## API

所有响应体为 JSON；错误统一为：

```json
{ "error": { "code": "not_found", "message": "资源不存在" } }
```

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| `GET` | `/health` | 存活探针（含数据库连通性） |
| `GET` | `/about` | 服务信息与端点清单 |
| `GET` | `/api/v1/tasks` | 列表，支持 `status` `q` `limit` `offset` |
| `POST` | `/api/v1/tasks` | 新建，成功返回 `201` |
| `GET` | `/api/v1/tasks/{id}` | 按 id 查询 |
| `PATCH` | `/api/v1/tasks/{id}` | 局部更新 |
| `DELETE` | `/api/v1/tasks/{id}` | 删除，成功返回 `204` |
| `GET` | `/api/v1/stats` | 按状态汇总数量 |

任务字段：

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `id` | string | UUID，对外标识（自增主键不外泄） |
| `title` | string | 必填，≤ 200 字符 |
| `description` | string \| null | 可选，≤ 2000 字符 |
| `status` | string | `todo` / `in_progress` / `done` / `cancelled` |
| `priority` | number | `0`–`4`，默认 `2` |
| `due_at` | string \| null | RFC3339 时间 |
| `created_at` / `updated_at` | string | RFC3339 时间，服务端维护 |

状态码：`400` 参数非法、`404` 不存在、`409` 冲突、`500` 服务端错误。

### 示例

```bash
# 新建
curl -X POST http://127.0.0.1:8080/api/v1/tasks \
  -H 'content-type: application/json' \
  -d '{"title":"写 README","description":"补充 API 文档","priority":3}'

# 列表（进行中 + 关键字）
curl 'http://127.0.0.1:8080/api/v1/tasks?status=in_progress&q=readme&limit=20'

# 局部更新：只改状态，description 传 null 表示清空，未提及的字段不变
curl -X PATCH http://127.0.0.1:8080/api/v1/tasks/<id> \
  -H 'content-type: application/json' \
  -d '{"status":"done","description":null}'

# 删除
curl -X DELETE http://127.0.0.1:8080/api/v1/tasks/<id>

# 统计
curl http://127.0.0.1:8080/api/v1/stats
```

## 项目结构

```
rust-web/
├── build.rs                     # 让 migrations/ 的改动触发重编译
├── migrations/
│   └── 0001_init.sql            # 表结构 + 索引（编译期内嵌）
├── static/
│   └── index.html               # 单文件任务看板（无构建步骤）
├── src/
│   ├── main.rs                  # 入口：配置 → 连接池 → 迁移 → 监听 → 优雅关闭
│   ├── lib.rs                   # AppState / AppOptions / build_router（便于测试）
│   ├── config.rs                # 环境变量 → Config
│   ├── db.rs                    # 连接池（WAL/外键/busy_timeout）与迁移
│   ├── error.rs                 # AppError → JSON 错误响应
│   ├── models.rs                # 领域模型、请求 DTO、校验、分页
│   ├── repo.rs                  # 数据访问层（唯一写 SQL 的地方）
│   └── routes/
│       ├── health.rs            # /health /about
│       └── tasks.rs             # /api/v1/*
└── tests/
    └── api.rs                   # 8 个端到端集成测试
```

## 设计说明

**为什么用 `id`（UUID）而不是自增主键？**
自增主键会泄露业务量、容易被遍历。库里保留 `id` 作为聚簇友好的主键，对外只暴露 `public_id`。

**为什么 `repo` 里不直接用 `sqlx::query!` 宏？**
`query!` 需要在编译期连接数据库（或维护 `.sqlx` 离线缓存）。这里统一使用运行时 API（`query_as` / `QueryBuilder`），`cargo build` 零外部依赖；代价是 SQL 错误从编译期推迟到运行期，因此集成测试是必需的。

**为什么动态 SQL 用 `QueryBuilder`？**
过滤条件是可选的，拼字符串再绑定既繁琐又容易出错。`QueryBuilder` 负责拼装与占位符编号；注意 `Separated` 每次调用都会补一个分隔符，所以「列名 + 占位符」要写成 `push("col =").push_bind_unseparated(value)`。

**时间戳为什么由 Rust 生成而不是数据库默认值？**
SQLite 没有原生的日期类型，统一由应用写入 RFC3339 文本，可以保证跨库一致，也便于测试注入。

## 部署

```bash
docker build -t rust-web .
docker run --rm -p 8080:8080 -v rust-web-data:/app/data rust-web
```

多阶段构建：SQLite 以 bundled 方式静态链接，运行镜像只有二进制与必要系统库。
生产环境请把 `CORS_PERMISSIVE` 设为 `false`，并按需收窄来源。

## License

MIT
