//! # Rust包和模块系统
//!
//! 本文件介绍Rust中的包(Package)、箱(Crate)、模块(Module)和路径(Path)系统

/// 这个函数演示了Rust中的包和模块系统
fn main() {
    // ===== 包和箱 =====
    println!("===== 包和箱 =====");
    // 包(Package)：一个或多个提供功能的箱的集合，包含Cargo.toml文件
    // 箱(Crate)：一个模块树，可以产生库或可执行文件
    // - 二进制箱(Binary Crate)：编译后产生可执行程序
    // - 库箱(Library Crate)：定义供其他程序使用的功能，不能直接执行
    
    println!("当前我们正在一个二进制箱中运行");
    
    // ===== 定义模块 =====
    println!("\n===== 定义模块 =====");
    
    // 模块是在箱内组织代码的方式
    // 使用mod关键字定义模块
    mod front_of_house {
        // 嵌套模块
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("添加到等待列表");
            }
            
            fn seat_at_table() {
                println!("安排入座");
            }
        }
        
        mod serving {
            fn take_order() {
                println!("接受订单");
            }
            
            fn serve_order() {
                println!("上菜");
            }
            
            fn take_payment() {
                println!("收款");
            }
        }
    }
    
    // ===== 路径 =====
    println!("\n===== 路径 =====");
    
    // 使用路径访问模块中的项
    // 1. 绝对路径：从箱根开始，使用箱名或字面值crate
    // 2. 相对路径：从当前模块开始，使用self、super或当前模块的标识符
    
    // 使用绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    
    // 使用相对路径
    front_of_house::hosting::add_to_waitlist();
    
    // ===== 私有性规则 =====
    println!("\n===== 私有性规则 =====");
    // 1. 所有项（函数、方法、结构体、枚举、模块和常量）默认是私有的
    // 2. 可以使用pub关键字使项变为公有
    // 3. 不允许使用定义在当前模块的子模块中的私有代码
    // 4. 允许使用任何定义在父模块或当前模块中的代码
    
    // front_of_house::serving::take_order(); // 错误：serving是私有的
    
    // ===== super关键字 =====
    println!("\n===== super关键字 =====");
    // super关键字用于访问父模块中的项
    
    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            // 使用super访问父模块
            super::deliver_order();
        }
        
        fn cook_order() {
            println!("烹饪订单");
        }
        
        // 公有结构体与字段
        pub struct Breakfast {
            pub toast: String,      // 公有字段
            seasonal_fruit: String, // 私有字段
        }
        
        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("桃子"),
                }
            }
        }
        
        // 公有枚举
        pub enum Appetizer {
            Soup,     // 枚举变体默认是公有的
            Salad,
        }
    }
    
    fn deliver_order() {
        println!("配送订单");
    }
    
    // 使用公有结构体和其公有字段
    let mut meal = back_of_house::Breakfast::summer("黑麦");
    meal.toast = String::from("小麦");
    println!("我要{}吐司面包", meal.toast);
    
    // meal.seasonal_fruit = String::from("蓝莓"); // 错误：seasonal_fruit是私有的
    
    // 使用公有枚举
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    
    // ===== use关键字 =====
    println!("\n===== use关键字 =====");
    // use关键字可以将路径引入作用域
    
    // 使用绝对路径
    use crate::front_of_house::hosting;
    
    // 现在可以直接使用hosting模块
    hosting::add_to_waitlist();
    
    // 使用as关键字提供新名称
    use crate::back_of_house::Appetizer as Starter;
    let order3 = Starter::Soup;
    
    // 重导出名称
    mod customer {
        pub use crate::front_of_house::hosting;
        
        pub fn eat_at_restaurant() {
            hosting::add_to_waitlist();
        }
    }
    
    // 使用嵌套路径整理大量use语句
    // 而不是:
    // use std::io;
    // use std::io::Write;
    // 可以使用:
    use std::io::{self, Write};
    
    // 通配符
    // use std::collections::*; // 将所有公有项引入作用域
    
    // ===== 将模块拆分为不同文件 =====
    println!("\n===== 模块文件系统 =====");
    // 在实际项目中，当模块变大时，可能需要将它们移动到单独的文件中
    // 例如：
    // src/
    //  ├── main.rs
    //  ├── front_of_house.rs
    //  └── front_of_house/
    //      └── hosting.rs
    
    // 在main.rs中：
    // mod front_of_house; // 告诉Rust在另一个同名文件中查找模块内容
    // use crate::front_of_house::hosting;
    
    // 在front_of_house.rs中：
    // pub mod hosting; // 告诉Rust在front_of_house/hosting.rs中查找模块内容
    
    // 在front_of_house/hosting.rs中：
    // pub fn add_to_waitlist() {
    //     println!("添加到等待列表");
    // }
    
    println!("模块系统是Rust代码组织的基础");
}

// 要运行这个文件，可以使用命令：
// cargo run --bin 06_packages_modules