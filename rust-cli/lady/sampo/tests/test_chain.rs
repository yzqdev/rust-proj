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
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}