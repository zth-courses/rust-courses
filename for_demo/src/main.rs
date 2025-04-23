fn main() {
    println!("Hello, world!");
    let _str: &str = "hello, hello";
    while_fn();
    for_fn();
    loop_fn();
}

// while循环
fn while_fn () {
    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT while loop")
}

// for循环
fn for_fn () {
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为：{}", i)
    }
}
// loop循环, 无限循环语句
//loop 循环可以通过 break 关键字类似于 return 一样使整个循环退出并给予外部一个返回值
// fn loop_fn () {
//     let s = ['R', 'U', 'N', 'o', 'O', 'B'];
//     let mut i = 0;
//     loop {
//         let ch = s[i];
//         if ch == 'o' {
//             println!("退出loop");
//             break ;
//         }
//         println!("\'{}\'", ch);
//         i += 1;
//     }
// }
fn loop_fn () {
    let s = ['R', 'U', 'N', 'o', 'O', 'B'];
    let mut i = 0;
    let index = loop {
        let ch = s[i];
        if ch == 'o' {
            println!("退出loop");
            break i;
        }
        println!("\'{}\'", ch);
        i += 1;
    };
    println!(" \'O\' 的索引为 {}", index);
}
