//! # Rust常见集合
//!
//! 本文件介绍Rust中的常见集合类型：Vector、String和HashMap

/// 这个函数演示了Rust中的常见集合类型及其操作
fn main() {
    // ===== Vector =====
    println!("===== Vector =====");
    
    // 创建空vector
    let v: Vec<i32> = Vec::new();
    println!("空vector: {:?}", v);
    
    // 使用宏创建vector
    let v = vec![1, 2, 3, 4, 5];
    println!("使用宏创建的vector: {:?}", v);
    
    // 更新vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("添加元素后的vector: {:?}", v);
    
    // 读取vector元素
    let v = vec![1, 2, 3, 4, 5];
    
    // 使用索引访问（可能会导致程序崩溃）
    let third = &v[2];
    println!("第三个元素: {}", third);
    
    // 使用get方法（返回Option<&T>）
    match v.get(2) {
        Some(third) => println!("第三个元素: {}", third),
        None => println!("没有第三个元素"),
    }
    
    // 尝试访问不存在的元素
    match v.get(100) {
        Some(element) => println!("第101个元素: {}", element),
        None => println!("没有第101个元素"),
    }
    
    // 遍历vector中的元素
    let v = vec![100, 32, 57];
    for i in &v {
        println!("值: {}", i);
    }
    
    // 遍历并修改vector中的元素
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 使用解引用运算符*来修改值
    }
    println!("修改后的vector: {:?}", v);
    
    // 使用枚举来存储不同类型的值
    println!("\n===== 使用枚举存储多种类型 =====");
    
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    // 处理不同类型的值
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("整数: {}", i),
            SpreadsheetCell::Float(f) => println!("浮点数: {}", f),
            SpreadsheetCell::Text(s) => println!("文本: {}", s),
        }
    }
    
    // ===== String =====
    println!("\n===== String =====");
    
    // 创建新的空String
    let mut s = String::new();
    println!("空字符串: '{}'", s);
    
    // 从字符串字面值创建String
    let data = "initial contents";
    let s = data.to_string();
    println!("从字面值创建: '{}'", s);
    
    // 也可以直接使用String::from
    let s = String::from("initial contents");
    println!("使用from创建: '{}'", s);
    
    // String可以包含任何有效的UTF-8数据
    let hello = String::from("你好");
    println!("UTF-8字符串: '{}'", hello);
    
    // 更新字符串
    let mut s = String::from("foo");
    s.push_str("bar"); // 追加字符串切片
    println!("追加后: '{}'", s);
    
    let mut s = String::from("lo");
    s.push('l'); // 追加单个字符
    println!("追加字符后: '{}'", s);
    
    // 使用+运算符或format!宏连接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意s1被移动了，不能继续使用
    println!("连接后: '{}'", s3);
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    // 使用format!宏（不会获取任何参数的所有权）
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("使用format!: '{}'", s);
    println!("原字符串仍可用: '{}', '{}', '{}'", s1, s2, s3);
    
    // 字符串索引
    let s = String::from("hello");
    // let h = s[0]; // 错误：Rust不支持通过索引访问字符串
    
    // 字符串切片
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 注意：这里切片的是字节，而不是字符
    println!("切片: '{}'", s); // 输出：Зд
    
    // 遍历字符串的方法
    println!("\n===== 遍历字符串 =====");
    
    // 按字符遍历
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!();
    
    // 按字节遍历
    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }
    println!();
    
    // ===== HashMap =====
    println!("\n===== HashMap =====");
    
    use std::collections::HashMap;
    
    // 创建新的空HashMap
    let mut scores = HashMap::new();
    
    // 插入键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("分数表: {:?}", scores);
    
    // 从两个vector创建HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    
    let mut scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("从vector创建的HashMap: {:?}", scores);
    
    // 哈希映射和所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name和field_value在这里不再有效
    
    // 访问哈希映射中的值
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    
    match score {
        Some(s) => println!("{}队的分数: {}", team_name, s),
        None => println!("{}队没有分数", team_name),
    }
    
    // 遍历哈希映射
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // 更新哈希映射
    println!("\n===== 更新HashMap =====");
    
    // 覆盖一个值
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 覆盖之前的值
    
    println!("覆盖后的分数: {:?}", scores);
    
    // 只在键没有对应值时插入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // 不会改变Blue的值
    
    println!("使用entry后的分数: {:?}", scores);
    
    // 根据旧值更新一个值
    let text = "hello world wonderful world";
    
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("单词计数: {:?}", map);
}

// 要运行这个文件，可以使用命令：
// cargo run --bin 05_collections