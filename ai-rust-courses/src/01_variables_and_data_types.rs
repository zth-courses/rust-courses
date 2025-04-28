//! # Rust变量和数据类型
//!
//! 本文件介绍Rust中的变量声明和基本数据类型

/// 这个函数演示了Rust中的变量声明和基本数据类型
fn main() {
    // 在Rust中，变量默认是不可变的（immutable）
    let x = 5;
    println!("x的值是: {}", x);
    
    // 如果需要可变变量，使用mut关键字
    let mut y = 10;
    println!("y的初始值是: {}", y);
    y = 20; // 可以修改可变变量的值
    println!("y的新值是: {}", y);
    
    // 变量遮蔽(Shadowing) - 可以声明同名新变量
    let z = 30;
    println!("z的初始值是: {}", z);
    let z = z + 10; // 这里创建了一个新变量z，而不是修改原来的z
    println!("z的新值是: {}", z);
    
    // 常量声明 - 使用const关键字，必须指定类型，命名通常使用大写
    const MAX_POINTS: u32 = 100_000;
    println!("常量值: {}", MAX_POINTS);
    
    // ===== 基本数据类型 =====
    
    // 整数类型
    let a: i8 = -10;    // 有符号8位整数，范围: -128 到 127
    let b: u8 = 10;     // 无符号8位整数，范围: 0 到 255
    let c: i16 = 1000;  // 有符号16位整数
    let d: u16 = 1000;  // 无符号16位整数
    let e: i32 = 10000; // 有符号32位整数（默认整数类型）
    let f: u32 = 10000; // 无符号32位整数
    let g: i64 = 10000; // 有符号64位整数
    let h: u64 = 10000; // 无符号64位整数
    let i: i128 = 10000; // 有符号128位整数
    let j: u128 = 10000; // 无符号128位整数
    let k: isize = 10000; // 取决于系统架构的有符号整数
    let l: usize = 10000; // 取决于系统架构的无符号整数
    
    println!("整数类型示例: i8={}, u8={}, i32={}, u64={}", a, b, e, h);
    
    // 整数字面值
    let decimal = 98_222;      // 十进制
    let hex = 0xff;           // 十六进制
    let octal = 0o77;         // 八进制
    let binary = 0b1111_0000; // 二进制
    let byte = b'A';          // 字节(仅限u8)
    
    println!("整数字面值: 十进制={}, 十六进制={}, 八进制={}, 二进制={}, 字节={}", 
             decimal, hex, octal, binary, byte);
    
    // 浮点数类型
    let float_32: f32 = 3.14;      // 32位浮点数
    let float_64: f64 = 3.14159;   // 64位浮点数（默认浮点类型）
    
    println!("浮点数: f32={}, f64={}", float_32, float_64);
    
    // 布尔类型
    let is_active: bool = true;
    let is_greater = 10 > 5;  // 布尔表达式
    
    println!("布尔值: is_active={}, is_greater={}", is_active, is_greater);
    
    // 字符类型 - Rust的char类型是Unicode标量值，占用4字节
    let letter: char = 'A';
    let emoji: char = '😊';
    let chinese_char: char = '中';
    
    println!("字符: letter={}, emoji={}, chinese_char={}", letter, emoji, chinese_char);
    
    // 复合类型
    
    // 元组类型 - 固定长度，可以包含不同类型的值
    let tup: (i32, f64, char) = (500, 6.4, 'A');
    let (x, y, z) = tup;  // 解构元组
    println!("元组解构: x={}, y={}, z={}", x, y, z);
    println!("元组索引访问: {}, {}, {}", tup.0, tup.1, tup.2);
    
    // 数组类型 - 固定长度，元素类型必须相同
    let arr1 = [1, 2, 3, 4, 5]; // 类型推断为[i32; 5]
    let arr2: [i32; 5] = [1, 2, 3, 4, 5]; // 显式类型标注
    let arr3 = [3; 5]; // 等同于 [3, 3, 3, 3, 3]
    
    println!("数组: arr1[0]={}, arr2[1]={}, arr3[2]={}", arr1[0], arr2[1], arr3[2]);
    println!("数组长度: {}", arr1.len());
    
    // 字符串类型
    // Rust有两种主要的字符串类型：&str和String
    let s1: &str = "Hello"; // 字符串字面值，不可变，固定大小
    let s2 = String::from("World"); // String类型，可变，可增长
    let s3 = s1.to_string(); // 从&str转换为String
    let s4 = &s2[..]; // 从String获取&str切片
    
    println!("字符串: s1={}, s2={}, s3={}, s4={}", s1, s2, s3, s4);
}

// 要运行这个文件，可以使用命令：
// cargo run --bin 01_variables_and_data_types