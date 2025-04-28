//! # Rust所有权系统
//!
//! 本文件介绍Rust中的所有权(Ownership)概念，这是Rust最独特的特性之一

/// 这个函数演示了Rust中的所有权规则和相关概念
fn main() {
    // ===== 所有权规则 =====
    // 1. Rust中的每一个值都有一个被称为其所有者(owner)的变量
    // 2. 值在任一时刻有且只有一个所有者
    // 3. 当所有者(变量)离开作用域，这个值将被丢弃
    
    println!("===== 所有权基础 =====");
    {
        // s 在这里无效，它尚未声明
        let s = "hello"; // 从此处起，s 是有效的
        println!("字符串字面值: {}", s);
        // 可以对 s 进行操作
    } // 此作用域已结束，s 不再有效
    
    // 内存分配与释放
    println!("\n===== 内存分配与释放 =====");
    {
        // String类型需要在堆上分配内存
        let s = String::from("hello"); // s 是有效的
        println!("堆上的字符串: {}", s);
        // 当 s 离开作用域时，Rust自动调用drop函数，释放内存
    } // 此处内存自动释放
    
    // 变量与数据交互的方式：移动(Move)
    println!("\n===== 变量移动 =====");
    let s1 = String::from("hello");
    let s2 = s1; // s1的所有权移动到s2，s1不再有效
    
    // println!("s1: {}", s1); // 错误：s1的值已移动
    println!("s2: {}", s2); // 正确：s2现在拥有该值
    
    // 克隆(Clone)
    println!("\n===== 克隆 =====");
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深度复制堆上的数据
    
    println!("克隆后 s1: {}, s2: {}", s1, s2); // 两者都有效
    
    // 栈上数据的复制(Copy)
    println!("\n===== 栈上数据的复制 =====");
    let x = 5;
    let y = x; // 整数是Copy类型，x仍然有效
    
    println!("复制后 x: {}, y: {}", x, y);
    
    // 实现了Copy trait的类型在赋值时会复制而不是移动
    // 包括：
    // - 所有整数类型，如u32、i32、usize等
    // - 布尔类型bool
    // - 浮点类型f32和f64
    // - 字符类型char
    // - 元组，当且仅当其包含的类型也都实现了Copy
    
    // ===== 所有权与函数 =====
    println!("\n===== 所有权与函数 =====");
    let s = String::from("hello");  // s 进入作用域
    
    takes_ownership(s);             // s 的值移动到函数里
    // println!("s: {}", s);       // 错误：s的值已被移动
    
    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，但i32是Copy的，所以后面可继续使用x
    println!("x仍然有效: {}", x);    // 这行能正常工作
    
    // ===== 返回值与作用域 =====
    println!("\n===== 返回值与作用域 =====");
    let s1 = gives_ownership();     // gives_ownership 将返回值移给 s1
    println!("从函数获得所有权: {}", s1);
    
    let s2 = String::from("hello"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2);  // s2 被移动到函数里，函数返回值移给 s3
    println!("转移并返回所有权: {}", s3);
    // 这里 s2 已失效
    
    // ===== 引用与借用 =====
    println!("\n===== 引用与借用 =====");
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1); // 传递s1的引用而非所有权
    println!("字符串 '{}' 的长度是 {}", s1, len); // s1仍然有效
    
    // 可变引用
    println!("\n===== 可变引用 =====");
    let mut s = String::from("hello");
    change(&mut s); // 传递可变引用
    println!("修改后的字符串: {}", s);
    
    // 可变引用的限制：在特定作用域中，对于某一块数据，只能有一个可变引用
    // 这可以防止数据竞争
    let mut s = String::from("hello");
    
    {
        let r1 = &mut s;
        // let r2 = &mut s; // 错误：不能同时有两个可变引用
        println!("可变引用r1: {}", r1);
    } // r1在这里离开作用域，所以我们可以创建新的引用
    
    let r2 = &mut s; // 现在可以了
    println!("可变引用r2: {}", r2);
    
    // 不能同时拥有可变引用和不可变引用
    let mut s = String::from("hello");
    
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("不可变引用: {} {}", r1, r2);
    // 此处r1和r2不再使用
    
    let r3 = &mut s; // 现在可以了
    println!("可变引用: {}", r3);
    
    // 悬垂引用(Dangling References)
    println!("\n===== 避免悬垂引用 =====");
    let reference_to_nothing = no_dangle();
    println!("安全的引用: {}", reference_to_nothing);
    
    // 切片(Slice)类型
    println!("\n===== 字符串切片 =====");
    let s = String::from("hello world");
    
    let hello = &s[0..5];  // 或者写成 &s[..5]
    let world = &s[6..11]; // 或者写成 &s[6..]
    println!("切片: '{}' '{}'", hello, world);
    
    // 字符串字面值就是切片
    let s = "Hello, world!"; // s的类型是&str，它是一个指向二进制程序特定位置的切片
    println!("字符串字面值(切片): {}", s);
    
    // 字符串切片作为参数
    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("第一个单词: {}", word);
    
    // 直接对字符串字面值调用也可以
    let word = first_word("hello world");
    println!("字面值的第一个单词: {}", word);
    
    // 其他切片
    println!("\n===== 其他类型的切片 =====");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("数组切片: {:?}", slice);
}

// 接受所有权的函数
fn takes_ownership(some_string: String) {
    println!("获得所有权: {}", some_string);
} // 函数结束，some_string被释放

// 接受复制类型参数的函数
fn makes_copy(some_integer: i32) {
    println!("获得复制: {}", some_integer);
} // 函数结束，不会发生特殊操作

// 返回所有权的函数
fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string 进入作用域
    some_string // 返回 some_string，所有权转移给调用者
}

// 接受并返回所有权的函数
fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 返回 a_string，所有权转移给调用者
}

// 使用引用的函数 - 不获取所有权
fn calculate_length(s: &String) -> usize { // s是String的引用
    s.len()
} // 这里，s离开作用域，但它并不拥有引用值的所有权，所以不会释放任何东西

// 使用可变引用的函数
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 返回引用但不会造成悬垂引用的函数
fn no_dangle() -> String {
    let s = String::from("hello");
    s // 返回String本身而不是引用，所有权被移动出去
}

// 使用字符串切片的函数
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 要运行这个文件，可以使用命令：
// cargo run --bin 03_ownership