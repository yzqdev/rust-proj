# ---- 构建阶段 -------------------------------------------------------------
FROM rust:1-slim AS builder

# SQLx 的 bundled SQLite 需要 C 工具链
RUN apt-get update \
    && apt-get install -y --no-install-recommends gcc libc6-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 先复制清单，利用缓存预编译依赖
COPY Cargo.toml Cargo.lock build.rs ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && echo '' > src/lib.rs \
    && cargo build --release \
    && rm -rf src

COPY migrations ./migrations
COPY src ./src
COPY static ./static

# 触碰源文件，确保真正编译本项目代码
RUN touch src/main.rs src/lib.rs && cargo build --release

# ---- 运行阶段 -------------------------------------------------------------
FROM debian:stable-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 app

WORKDIR /app
COPY --from=builder /app/target/release/rust-web /usr/local/bin/rust-web
COPY --from=builder /app/static ./static

# 数据目录归属非 root 用户
RUN mkdir -p /app/data && chown -R app:app /app
USER app

ENV APP_HOST=0.0.0.0 \
    APP_PORT=8080 \
    DATABASE_URL=sqlite://data/app.db \
    STATIC_DIR=static \
    CORS_PERMISSIVE=false \
    RUST_LOG=info,rust_web=info

EXPOSE 8080
VOLUME ["/app/data"]

CMD ["rust-web"]
