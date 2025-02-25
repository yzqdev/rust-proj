use std::env;

#[test]
fn say_sth() {}

#[cfg(target_os = "windows")]
#[test]
fn look_at_this() {
    println!("{}", env::temp_dir().display());
    let env_vars = env::vars();
    println!("{}", env::var("APPDATA").expect("hello"))
}
