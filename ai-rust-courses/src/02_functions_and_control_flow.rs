//! # Rust函数和流程控制
//!
//! 本文件介绍Rust中的函数定义和各种流程控制结构

/// 这个函数演示了Rust中的函数定义和流程控制结构
fn main() {
    // 调用函数
    println!("函数调用示例:");
    say_hello();
    
    // 带参数的函数
    let result = add(5, 10);
    println!("5 + 10 = {}", result);
    
    // 带参数和类型注解的函数
    print_number(100);
    print_sum(10, 20);
    
    // ===== 流程控制 =====
    
    // if 表达式
    let number = 6;
    
    if number % 4 == 0 {
        println!("{} 能被4整除", number);
    } else if number % 3 == 0 {
        println!("{} 能被3整除", number);
    } else if number % 2 == 0 {
        println!("{} 能被2整除", number);
    } else {
        println!("{} 不能被4、3或2整除", number);
    }
    
    // if 是表达式，可以用在 let 语句中
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("if表达式赋值: number = {}", number);
    
    // 循环 - loop
    println!("\nloop循环示例:");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("  loop计数: {}", counter);
        
        if counter == 3 {
            // 使用break返回值
            break counter * 2;
        }
    };
    println!("loop返回值: {}", result);
    
    // 循环标签 - 用于嵌套循环中的break和continue
    println!("\n循环标签示例:");
    'outer: loop {
        println!("  进入外层循环");
        
        let mut inner_count = 0;
        loop {
            inner_count += 1;
            println!("    内层循环计数: {}", inner_count);
            
            if inner_count == 2 {
                println!("    跳出内层循环");
                break;
            }
            
            if counter > 5 {
                println!("    跳出外层循环");
                break 'outer;
            }
        }
        
        counter += 1;
        if counter >= 5 {
            println!("  外层循环结束");
            break;
        }
    }
    
    // while 循环
    println!("\nwhile循环示例:");
    let mut number = 3;
    while number != 0 {
        println!("  倒计时: {}", number);
        number -= 1;
    }
    println!("  发射!");
    
    // for 循环 - 最常用的循环结构
    println!("\nfor循环示例:");
    
    // 遍历范围
    println!("  遍历范围:");
    for i in 1..4 { // 不包含上界
        println!("    计数: {}", i);
    }
    
    // 遍历范围（包含上界）
    println!("  遍历范围(包含上界):");
    for i in 1..=3 { // 包含上界
        println!("    计数: {}", i);
    }
    
    // 遍历数组
    println!("  遍历数组:");
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("    元素: {}", element);
    }
    
    // 遍历并获取索引
    println!("  遍历并获取索引:");
    for (index, value) in arr.iter().enumerate() {
        println!("    索引 {} 的值是: {}", index, value);
    }
    
    // match 表达式 - 强大的模式匹配
    println!("\nmatch表达式示例:");
    let dice_roll = 4;
    match dice_roll {
        1 => println!("  掷得1点"),
        2 => println!("  掷得2点"),
        3 => println!("  掷得3点"),
        4..=6 => println!("  掷得{}点", dice_roll),
        _ => println!("  无效的骰子点数"),
    }
    
    // if let 简化的匹配
    println!("\nif let示例:");
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("  值是3");
    }
}

/// 简单的无参数、无返回值函数
fn say_hello() {
    println!("  你好，Rust!");
}

/// 带参数和返回值的函数
/// 
/// 在Rust中，函数的返回值类型在箭头(->)后指定
fn add(a: i32, b: i32) -> i32 {
    // 注意：没有分号的表达式是返回值
    a + b
}

/// 打印一个数字的函数
fn print_number(x: i32) {
    println!("  数字是: {}", x);
}

/// 打印两个数字之和的函数
fn print_sum(x: i32, y: i32) {
    println!("  {} + {} = {}", x, y, x + y);
}

// 要运行这个文件，可以使用命令：
// cargo run --bin 02_functions_and_control_flow