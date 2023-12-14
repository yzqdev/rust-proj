#![allow(dead_code)]
use std::fmt::Display;
#[derive(Debug)]
enum FileState {
    Open,
    Closed,
}
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}
impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}
impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileState::Open => write!(f, "Open"),
            FileState::Closed => write!(f, "Closed"),
        }
    }
}
impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}
#[test]
fn main_test() {
    let f = File::new("test.txt");
    //Debug按照默认实现的打印
    //File { name: "test.txt", data: [], state: Closed }
    println!("{:?}", f);
    //自定义格式的打印，不需要添加任何#[derive(Debug)]标注
    //<test.txt (Closed)>
    println!("{}", f);
}