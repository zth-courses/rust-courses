//! # Rust结构体、枚举和模式匹配
//!
//! 本文件介绍Rust中的结构体(Struct)、枚举(Enum)和模式匹配

/// 这个函数演示了Rust中的结构体、枚举和模式匹配
fn main() {
    // ===== 结构体 =====
    println!("===== 结构体 =====");
    
    // 定义一个结构体
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    // 创建结构体实例
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // 修改结构体字段（需要整个实例是可变的）
    user1.email = String::from("anotheremail@example.com");
    println!("用户邮箱: {}", user1.email);
    
    // 使用函数创建结构体实例
    let user2 = build_user(
        String::from("user2@example.com"),
        String::from("user2"),
    );
    println!("用户名: {}, 是否活跃: {}", user2.username, user2.active);
    
    // 使用结构体更新语法从其他实例创建实例
    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        ..user1 // 其余值来自user1
    };
    println!("用户3: {}，登录次数: {}", user3.username, user3.sign_in_count);
    
    // 元组结构体 - 有名字的元组
    println!("\n===== 元组结构体 =====");
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("黑色RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("原点坐标: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // 类单元结构体 - 没有任何字段
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    // 这种结构体通常用于实现trait但不需要存储数据的情况
    
    // 使用结构体的例子：计算长方形面积
    println!("\n===== 结构体示例 =====");
    
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    // 为结构体实现方法
    impl Rectangle {
        // 关联函数（不以self为参数）- 类似于静态方法
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
        
        // 方法（第一个参数是self）
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        fn width(&self) -> bool {
            self.width > 0
        }
        
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "长方形面积: {} 平方像素",
        rect1.area()
    );
    
    if rect1.width() {
        println!("长方形宽度为 {}", rect1.width);
    }
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("rect1能容纳rect2吗? {}", rect1.can_hold(&rect2));
    println!("rect1能容纳rect3吗? {}", rect1.can_hold(&rect3));
    
    // 使用关联函数创建正方形
    let square = Rectangle::square(20);
    println!("正方形面积: {} 平方像素", square.area());
    
    // ===== 枚举 =====
    println!("\n===== 枚举 =====");
    
    // 定义枚举
    enum IpAddrKind {
        V4,
        V6,
    }
    
    // 使用枚举值
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    // 将数据附加到枚举变体
    enum IpAddr {
        V4(String),
        V6(String),
    }
    
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    
    // 不同变体可以有不同类型和数量的关联数据
    enum IpAddrDetailed {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddrDetailed::V4(127, 0, 0, 1);
    
    // 复杂的枚举示例
    enum Message {
        Quit,                       // 没有关联数据
        Move { x: i32, y: i32 },    // 匿名结构体
        Write(String),              // 包含一个String
        ChangeColor(i32, i32, i32), // 包含三个i32
    }
    
    // 为枚举实现方法
    impl Message {
        fn call(&self) {
            // 方法体
            println!("消息被调用");
        }
    }
    
    let m = Message::Write(String::from("hello"));
    m.call();
    
    // Option枚举 - Rust标准库中的重要枚举
    println!("\n===== Option枚举 =====");
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
    
    // Option<T>和T是不同的类型，不能直接相加
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    // let sum = x + y; // 错误：不能将Option<i8>和i8相加
    
    // 需要先处理Option，提取出其中的值
    let sum = x + y.unwrap_or(0);
    println!("x + y = {}", sum);
    
    // ===== match控制流 =====
    println!("\n===== match控制流 =====");
    
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ... 其他州
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
    
    let penny_value = value_in_cents(Coin::Penny);
    println!("一分硬币的值: {}分", penny_value);
    
    let quarter_value = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("25分硬币的值: {}分", quarter_value);
    
    // 匹配Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
    
    // 匹配必须穷尽所有可能性
    // 使用_通配符处理其他情况
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("其他值: {}", some_u8_value),
    }
    
    // if let 简洁控制流
    println!("\n===== if let =====");
    
    let some_u8_value = Some(3);
    
    // 使用match
    match some_u8_value {
        Some(3) => println!("match: 值是三!"),
        _ => (),
    }
    
    // 使用if let（更简洁）
    if let Some(3) = some_u8_value {
        println!("if let: 值是三!");
    }
    
    // if let也可以包含else
    let coin = Coin::Penny;
    
    if let Coin::Quarter(state) = coin {
        println!("25分硬币来自{:?}州!", state);
    } else {
        println!("不是25分硬币，而是值为{}分的硬币", value_in_cents(coin));
    }
}

// 创建User实例的函数
fn build_user(email: String, username: String) -> User {
    User {
        email,      // 字段初始化简写
        username,   // 字段初始化简写
        active: true,
        sign_in_count: 1,
    }
}

// 结构体定义（在函数外部）
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 要运行这个文件，可以使用命令：
// cargo run --bin 04_structs_enums_pattern_matching