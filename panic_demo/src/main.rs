use std::fs::File;
use std::io::{prelude::*, ErrorKind};

fn main() {
    // panic!("crash and burn");

    // 创建并写入一个文件
    let mut fs = File::create("hello.txt").unwrap();
    fs.write_all(b"hello world").unwrap();

    // 读取文件
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // 对错误分类处理
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件失败: {:?}", e),
            },
            other_error => panic!("打开文件失败: {:?}", other_error),
        },
    };
    println!("{:?}", f)
}
     