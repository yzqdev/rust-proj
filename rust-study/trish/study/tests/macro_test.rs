
use study::{create_function, print_result, say_hello};

#[test]
fn hello_macro(){
    // 这个调用将会展开成 `println("Hello");`!
    say_hello!()
}
#[test]
fn create_fun_test() {
    // 借助上述宏来创建名为 `foo` 和 `bar` 的函数。
    create_function!(foo);
    create_function!(bar);
    foo();
    bar();

    print_result!(1u32 + 1);

    // 回想一下，代码块也是表达式！
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}
