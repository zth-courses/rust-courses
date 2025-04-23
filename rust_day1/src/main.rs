/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let x = add(1, 2);
///
/// ```

fn main() {
    let mut name = "rzx";
    let age = 30 - 1;
    let _male = false;
    println!("Hello, {0}, age is {1}", name, age);
    name = "rzx007";
    println!("my fullName is {}", name);

    // 数组
    let _arr = [1, 2, 3, 4];
    another_function(23, 12);

    enum_function();

    arithmetic();

    composite_type();

    let mut number = if_else();
    println!("if_else返回值为{number}");
    number = 10;
    println!("修改后number的值为{number}");

    let _res = loop_fn();

    while_fn();

    for_fn();
}

// fn <函数名> ( <参数> ) <函数体>
fn another_function(x: i32, y: i32) {
    println!("{0}, {1}", x, y);
    // Rust 中可以在一个用 {} 包括的块里编写一个较为复杂的表达式：
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}

// 枚举
fn enum_function() {
    #[derive(Debug)]
    enum Book {
        Papery(u32),
        Electronic(String),
    }
    let book = Book::Papery(1001);
    let ebook = Book::Electronic(String::from("url://..."));
    println!("{:?}", book);
}

// 运算符
fn arithmetic() {
    let _sum = 5 + 10;

    let _difference = 95.5 - 4.3;

    let _product = 4 * 30;

    let _quotient = 56.7 / 32.2;

    let _reminder = 54 % 5;
}

// 复合类型
fn composite_type() {
    // 元组(tuple)
    // 元组有着固定的长度。而且一旦定义，就不能再增长或缩小。
    // 元组的下标从 0 开始。
    let tup: (i32, f64, u8) = (500, 63.2, 23);
    let (x, y, z) = tup;
    println!("{x}, {y},{z}, {}", tup.0);

    // 数组
    let a = [1, 2, 3, 4, 5];
    // a 是一个长度为 5 的整型数组

    let _b = ["January", "February", "March"];
    // b 是一个长度为 3 的字符串数组

    let _c: [i32; 5] = [1, 2, 3, 4, 5];
    // c 是一个长度为 5 的 i32 数组

    let _d = [3; 5];
    // 等同于 let d = [3, 3, 3, 3, 3];

    let _first = a[0];
    let _second = a[1];
    // 数组访问

    // a[0] = 123; // 错误：数组 a 不可变
    let mut a = [1, 2, 3];
    a[0] = 4; // 正确
    
    println!("{:?}", a)
}

//控制流：if else
fn if_else() -> i32 {
    let x = 4;
    if x > 5 {
        println!("x 大于 5")
    } else if x < 4 {
        println!("x 小于 4")
    } else {
        println!("x 等于 4")
    }
    // 相当于三目运算
    if x == 4 {
        4
    } else {
        5
    }
}

// loop循环
fn loop_fn() -> i32 {
    // loop 一直反复执行代码块,知道你让他停止
    let mut x = 1;
    loop {
        x += 1;
        println!("{x}");
        if x >= 20 {
            break x; // break停止，并可以返回值
        }
    }
}

fn while_fn() {
    let mut number = 1;
    while number != 20 {
        println!("x的值{number}");
        number += 1;
    }
    println!("END");
}

fn for_fn() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("the value is {element}")
    }

    // 倒序循环
    for number in (1..4).rev() {
        println!("倒计时{number}!")
    }
    println!("lift-off")
}
