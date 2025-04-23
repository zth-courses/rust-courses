#![allow(unused_variables)]
fn main() {
    // 所有权
    // 基本类型,变量的移动直接复制
    let s = String::from("hello");
    // s 被声明有效

    takes_ownership(s);
    // s 的值被当作参数传入函数
    // 所以可以当作 s 已经被移动，从这里开始已经无效

    let x = 5;
    // x 被声明有效

    makes_copy(x);
    // x 的值被当作参数传入函数
    // 但 x 是基本类型，依然有效
    // 在这里依然可以使用 x 却不能使用 s
}  // 函数结束, x 无效, 然后是 s. 但 s 已被移动, 所以不用被释放

fn takes_ownership(strs: String) {
    println!("{}", strs);
}

fn makes_copy(num: i32) {
    println!("{}", num);
}