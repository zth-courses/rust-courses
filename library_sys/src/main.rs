// 图书馆管理终端应用
use std::{io, env};

fn main() {
    enum FeaturesOptions {
        ADD(u8),
        EDIT(u8),
        DELETE(u8),
        GET(u8),
        QUIT(u8),
    }


    fn math_feature(num: u8) -> FeaturesOptions {
        match num {
            1 => FeaturesOptions::ADD(1),
            _  => {
                println!("未知操作");
                FeaturesOptions::QUIT(1)
            },
        }
    }

   // math_feature(1);

    println!(r"
    0. 退出系统；
    1. 新增图书；
    2. 修改读书；
    3. 删除图书；
    4. 查询图书.
    ");



   loop {
       // 获取终端输入
       let mut feature_key: String = String::new();
       // 获取终端输入
       io::stdin().read_line(&mut feature_key).unwrap();
       // 尝试将输入转换为数组
       let feature_key: u8 = match feature_key.trim().parse() {
           Ok(num) => {
               num
           },
           Err(_) => {
               println!("请输入数字");
               continue;
           }
       };
       math_feature(feature_key);
   }
}
