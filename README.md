# rust教程

## cargo workspace
直接在顶部的Cargo.toml添加
```toml
[workspace]

members = [
    "adder",
]


```

然后

### 创建二进制

```shell
cargo new adder
```
运行`cargo build`构建工作空间
```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target

```
### 创建lib
```shell
cargo new add_one --lib
```

为了在顶层 add 目录运行二进制 crate，可以通过 -p 参数和包名称来运行 cargo run 指定工作空间中我们希望使用的包
```shell
 cargo run -p adder
 
# 添加参数
 cargo run -p cli-tool --  example

```

测试
```shell
cargo test -p add_one
```