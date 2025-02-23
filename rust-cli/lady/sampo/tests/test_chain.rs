#[test]
fn test_iter(){
    println!("hello world")
}
#[test]
fn test_life(){
      struct Str<'a>  {
        content: &'a str
    }
    let s = Str {
        content: "string_slice" 
    };
    println!("s.content = {}", s.content);
}
fn longer(s1: &str, s2: &str) -> &str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}