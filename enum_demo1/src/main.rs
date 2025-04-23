fn main() {
    #[derive(Debug)]
    // 将数据附加到枚举的变体中
    //可以直接可枚举设值
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let ip_v4 = IpAddrKind::V4(127, 0, 0, 1);
    let ip_v6 = IpAddrKind::V6(String::from("::1"));
    println!("ipV4: {:?}", ip_v4);
    println!("ipV6: {:?}", ip_v6);

    // Option<T> 枚举
    // Option<T> 是一个标准库中的枚举，它定义如下：
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // Some(T) 表示存在某个 T 值，而 None 则表示值不存在。
    // Option<T> 是一个泛型，因为可能有各种类型的值，比如 i32、f64、String 等。
    // Option<T> 和 T 有点类似，但 Option<T> 是一个枚举，它的两个变体分别是 Some(T) 和 None。
    // Option<T> 有一个方法叫做 unwrap，它的作用是返回 Option<T> 中的 T 值。
    // 如果 Option<T> 的值是 None，unwrap 会导致程序崩溃并显示一个错误信息。
    // 如果 Option<T> 的值是 Some(T)，unwrap 会返回 T 的值。
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
    // println!("sum: {:?}", sum);
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y.unwrap();
    // println!("sum: {:?}", sum);
    // let x: i8 = 5;
    // let y: Option<i8> = None;
    // let sum = x + y.unwrap();
    // println!("sum: {:?}", sum);
    // let x: i8 = 5;
    // let y: Option<i8> = None;
    // let sum = x + y.unwrap_or(0);
    // println!("sum: {:?}", sum);
    // let x: i8 = 5;
    // let y: Option<i8> = None;
    // let sum = x + y.unwrap_or_else(|| 0);
    // println!("sum: {:?}", sum);
    // let x: i8 = 5;
    // let y: Option<i8> = None;
    // let sum = x + y.unwrap_or_else(|| {
    //     println!("y is None");
    //     0
    // });
    // println!("sum: {:?}", sum);
    
}
