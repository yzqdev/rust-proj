fn main() {
    // migrations/ 里的 SQL 改动不会自动触发重新编译，这里显式声明依赖。
    println!("cargo:rerun-if-changed=migrations");
}
