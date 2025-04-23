mod borrowed;
mod fss;
mod guess;
mod myenum;
use std::io;
fn main() {
    let _strs = String::from("begining");
    // 接收命令行参数
    let args = std::env::args();
    for arg in args {
        println!("{:?}", arg);
    }

    // 读取文件
    let fsa = fss::Fss::new();
    fsa.read();
    fss::Fss::read_file("./Cargo.toml");
    borrowed::Borr::init();

    // 枚举类
    myenum::Enum::init();

    // 随机数
    let x = guess::Guess::gen_random();

    println!("随机数是: {}", x);
    println!("猜数！");

    println!("猜测一个数");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜的数字是: {}", guess);
}
