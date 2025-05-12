use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    // 接收命令行参数
    let args = std::env::args();
    for arg in args {
        println!("{:?}", arg);
    }

    println!("猜字游戏！");
    // 生成一个随机数
    let x: u32 = rand::rng().random_range(1..101);
    println!("随机数是：{}", x);

    loop {
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字");
                continue;
            }
        };
        println!("你猜的数字是：{}", guess);
        match guess.cmp(&x) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
        }
    }

    // 判断一个数是否大于5，赋值给x
    let _x1 = if x > 5 { 1 } else { 0 };
}
