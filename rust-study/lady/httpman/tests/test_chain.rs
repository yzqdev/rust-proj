use httpman::{create_function, find_min, print_result, split_word};
#[test]
fn test_iter() {
    println!("hello world");
    create_function!(failed);
    failed();
    print_result!("hello world");
    split_word!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    split_word!(true; or false);
}
#[test]
fn find_min_num(){
     println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2 , 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}