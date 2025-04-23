fn main() {
    let str: String = String::from("你好");
    let len: usize = str.len();
    println!("{} of string is {}", str, len);

    //遍历标量值
    for c in str.chars() {
        println!("{}", c);
    }

    //遍历字节
    for i in str.bytes() {
        println!("{}", i);
    }

    // 字符串切片
    let str: String = String::from("abcdefg");
    let s: &str = &str[0..3];
    println!("{}", s);

    // 字符串拼接
    let s1: String = String::from("hello, ");
    let s2: String = String::from("world!");
    let s3: String = s1 + &s2;
    // 此时s2已失效
    println!("{}", s3)
}
