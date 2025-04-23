fn main() {
    println!("Hello, world!");
    println!("{}", add(1, 2))
}
/// fn <函数名> ( <参数> ) <函数体>
fn add(a: i32, b:i32) ->i32 {
    a + b
}