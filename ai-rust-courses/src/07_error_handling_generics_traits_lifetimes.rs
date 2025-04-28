//! # Rust错误处理、泛型、Trait和生命周期
//!
//! 本文件综合介绍Rust中的错误处理机制、泛型、Trait和生命周期概念

/// 这个函数演示了Rust中的错误处理、泛型、Trait和生命周期
fn main() {
    // ===== 错误处理 =====
    println!("===== 错误处理 =====");
    
    // Rust将错误分为两大类：可恢复错误和不可恢复错误
    
    // 不可恢复错误 - panic!
    println!("\n--- 不可恢复错误 ---");
    // panic!("崩溃并退出"); // 取消注释将导致程序崩溃
    
    // 可以设置RUST_BACKTRACE=1环境变量查看详细的堆栈跟踪
    
    // 可恢复错误 - Result枚举
    println!("\n--- 可恢复错误 ---");
    // enum Result<T, E> {
    //     Ok(T),  // 操作成功，包含成功值
    //     Err(E), // 操作失败，包含错误信息
    // }
    
    // 使用Result处理可能失败的操作
    use std::fs::File;
    use std::io::{self, ErrorKind, Read};
    
    // 尝试打开文件
    let file_result = File::open("hello.txt");
    
    // 使用match处理Result
    let file = match file_result {
        Ok(file) => {
            println!("文件打开成功");
            file
        },
        Err(error) => {
            println!("打开文件失败: {:?}", error);
            
            // 可以根据错误类型进一步处理
            match error.kind() {
                ErrorKind::NotFound => {
                    println!("文件不存在，尝试创建...");
                    match File::create("hello.txt") {
                        Ok(fc) => {
                            println!("文件创建成功");
                            fc
                        },
                        Err(e) => {
                            println!("创建文件失败: {:?}", e);
                            panic!("无法创建文件");
                        },
                    }
                },
                other_error => {
                    println!("其他错误: {:?}", other_error);
                    panic!("无法打开文件");
                },
            }
        },
    };
    
    // 使用unwrap和expect简化错误处理
    // unwrap: 如果Result是Ok，返回Ok中的值；如果是Err，调用panic!
    // let file = File::open("hello.txt").unwrap();
    
    // expect: 类似unwrap，但可以指定panic!的错误信息
    // let file = File::open("hello.txt").expect("无法打开hello.txt文件");
    
    // 错误传播
    println!("\n--- 错误传播 ---");
    
    // 使用?运算符简化错误传播
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();
        
        // 使用?运算符：如果Result是Err，立即返回该Err；否则获取Ok中的值
        let mut file = File::open("hello.txt")?;
        file.read_to_string(&mut username)?;
        
        Ok(username)
    }
    
    // 更简洁的链式调用
    fn read_username_from_file_short() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    
    // 最简洁的方式
    fn read_username_from_file_shortest() -> Result<String, io::Error> {
        std::fs::read_to_string("hello.txt")
    }
    
    // 调用返回Result的函数
    match read_username_from_file() {
        Ok(username) => println!("读取到用户名: {}", username),
        Err(e) => println!("读取用户名失败: {:?}", e),
    }
    
    // ===== 泛型 =====
    println!("\n===== 泛型 =====");
    
    // 泛型允许我们定义可以适用于多种类型的代码
    
    // 泛型函数
    println!("\n--- 泛型函数 ---");
    
    // 定义一个泛型函数，可以接受任何类型的参数
    fn print_value<T>(value: T) {
        println!("值: {:?}", value);
    }
    
    // 使用泛型函数（需要实现Debug trait）
    print_value("hello"); // T是&str类型
    print_value(5);       // T是i32类型
    print_value(3.14);    // T是f64类型
    
    // 返回两个值中较大的一个
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        
        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        
        largest
    }
    
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大的数字是: {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大的字符是: {}", result);
    
    // 泛型结构体
    println!("\n--- 泛型结构体 ---");
    
    // 定义一个包含任意类型的Point结构体
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    
    // 使用泛型结构体
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("整数点: {:?}", integer_point);
    println!("浮点数点: {:?}", float_point);
    
    // 多个泛型参数的结构体
    #[derive(Debug)]
    struct MixedPoint<T, U> {
        x: T,
        y: U,
    }
    
    let mixed_point = MixedPoint { x: 5, y: 4.0 };
    println!("混合类型点: {:?}", mixed_point);
    
    // 泛型枚举
    println!("\n--- 泛型枚举 ---");
    
    // Option和Result就是标准库中的泛型枚举
    enum Option<T> {
        Some(T),
        None,
    }
    
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    
    // 为泛型结构体实现方法
    println!("\n--- 泛型方法 ---");
    
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    
    // 为特定类型实现方法
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    
    let p_float = Point { x: 3.0_f32, y: 4.0_f32 };
    println!("到原点的距离: {}", p_float.distance_from_origin());
    
    // 方法使用不同的泛型参数
    impl<T, U> MixedPoint<T, U> {
        fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
            MixedPoint {
                x: self.x,
                y: other.y,
            }
        }
    }
    
    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };
    
    let p3 = p1.mixup(p2);
    println!("混合后的点: x = {}, y = {}", p3.x, p3.y);
    
    // ===== Trait（特质/特征） =====
    println!("\n===== Trait =====");
    
    // Trait定义了一组可以被共享的行为（类似于其他语言中的接口）
    
    // 定义一个Trait
    pub trait Summary {
        // 方法签名
        fn summarize(&self) -> String;
        
        // 带有默认实现的方法
        fn default_summary(&self) -> String {
            String::from("(阅读更多...)")
        }
    }
    
    // 为类型实现Trait
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, 作者 {} ({})", self.headline, self.author, self.location)
        }
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
        
        // 覆盖默认实现
        fn default_summary(&self) -> String {
            format!("来自 @{} 的推文", self.username)
        }
    }
    
    // 使用Trait
    let article = NewsArticle {
        headline: String::from("Rust 1.50发布"),
        location: String::from("全球"),
        author: String::from("Rust团队"),
        content: String::from("Rust 1.50带来了许多新特性..."),
    };
    
    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("我们刚刚发布了Rust 1.50!"),
        reply: false,
        retweet: false,
    };
    
    println!("文章摘要: {}", article.summarize());
    println!("文章默认摘要: {}", article.default_summary());
    println!("推文摘要: {}", tweet.summarize());
    println!("推文默认摘要: {}", tweet.default_summary());
    
    // Trait作为参数
    println!("\n--- Trait作为参数 ---");
    
    // 接受任何实现了Summary trait的类型
    pub fn notify(item: &impl Summary) {
        println!("突发新闻! {}", item.summarize());
    }
    
    notify(&article);
    notify(&tweet);
    
    // Trait约束语法
    pub fn notify_verbose<T: Summary>(item: &T) {
        println!("突发新闻! {}", item.summarize());
    }
    
    // 多个Trait约束
    // pub fn notify(item: &(impl Summary + Display)) {}
    // pub fn notify<T: Summary + Display>(item: &T) {}
    
    // 使用where子句简化Trait约束
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    //     where T: Display + Clone,
    //           U: Clone + Debug
    // {}
    
    // 返回实现了Trait的类型
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("当然，你知道的..."),
            reply: false,
            retweet: false,
        }
    }
    
    let summary_item = returns_summarizable();
    println!("返回的实现了Summary的项: {}", summary_item.summarize());
    
    // 使用Trait约束有条件地实现方法
    println!("\n--- 有条件的实现 ---");
    
    struct Pair<T> {
        x: T,
        y: T,
    }
    
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    
    // 只为那些实现了Display和PartialOrd的类型实现cmp_display方法
    impl<T: std::fmt::Display + std::cmp::PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("最大的成员是x = {}", self.x);
            } else {
                println!("最大的成员是y = {}", self.y);
            }
        }
    }
    
    let pair = Pair::new(10, 5);
    pair.cmp_display();
    
    // ===== 生命周期 =====
    println!("\n===== 生命周期 =====");
    
    // 生命周期是Rust的另一种泛型，它确保引用在我们需要它们的时候保持有效
    
    // 悬垂引用问题
    println!("\n--- 悬垂引用问题 ---");
    
    // 下面的代码会导致编译错误
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x; // x将在内部作用域结束时离开
    //     }
    //     println!("r: {}", r); // 错误：x已经不存在了
    // }
    
    // 生命周期注解语法
    println!("\n--- 生命周期注解 ---");
    
    // 生命周期参数名以撇号(')开头，通常使用小写字母，如'a
    // &i32        // 引用
    // &'a i32     // 带有显式生命周期的引用
    // &'a mut i32 // 带有显式生命周期的可变引用
    
    // 函数签名中的生命周期注解
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是: {}", result);
    
    // 生命周期的工作方式
    println!("\n--- 生命周期工作方式 ---");
    
    // 生命周期注解不改变引用的实际生命周期，只是帮助编译器检查引用的有效性
    
    // 结构体中的生命周期
    println!("\n--- 结构体中的生命周期 ---");
    
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    let novel = String::from("从前有一个人。他住在一个小村庄里...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt { part: first_sentence };
    
    println!("摘录: {}", excerpt.part);
    
    // 生命周期省略规则
    println!("\n--- 生命周期省略规则 ---");
    
    // 编译器使用三条规则来推断引用的生命周期：
    // 1. 每个引用参数都有自己的生命周期参数
    // 2. 如果只有一个输入生命周期参数，那么它被赋给所有输出生命周期参数
    // 3. 如果有多个输入生命周期参数，但其中一个是&self或&mut self，
    //    那么self的生命周期被赋给所有输出生命周期参数
    
    // 由于这些规则，以下函数不需要显式的生命周期注解：
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    // 静态生命周期
    println!("\n--- 静态生命周期 ---");
    
    // 'static生命周期表示引用在整个程序运行期间都有效
    // 所有的字符串字面值都有'static生命周期
    let s: &'static str = "我有静态生命周期.";
    println!("{}", s);
    
    // 结合泛型类型参数、trait约束和生命周期
    println!("\n--- 综合示例 ---");
    
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: std::fmt::Display,
    {
        println!("公告: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let result = longest_with_an_announcement(
        "Hello",
        "world",
        "今天是字符串比较日!",
    );
    println!("最长的字符串是: {}", result);
}

// 要运行这个文件，可以使用命令：
// cargo run --bin 07_error_handling_generics_traits_lifetimes