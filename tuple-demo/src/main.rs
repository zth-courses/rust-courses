fn main() {
    let tup: (i32, f64, u8) = (500, 12.02, 2);
    // 用模式匹配解构元组
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // 用.访问元组

    let ss = tup.0;
    let ss1 = tup.1;
    println!("{},{}", ss, ss1);

    // 元组的使用示例
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() 返回字符串的长度

        (s, length)
    }
}
