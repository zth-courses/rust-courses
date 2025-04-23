pub struct Enum {}

#[derive(Debug)]

pub enum Book {
    Papery(u32),
    Electronic(String),
}

impl Enum {
    pub fn init() {
        let book = Book::Papery(123);
        let _ele = Book::Electronic(String::from("你是"));
        match book {
            Book::Papery(i) => {
                println!("Papery book {}", i);
            }
            Book::Electronic(u) => {
                println!("E-book {}", u);
            }
        }
    }
    pub fn enum_str() {
        let t = "abc";
        match t {
            "abc" => println!("Yes"),
            _ => {} // 兜底
        }
    }
}
// 相当于switch case
// match 枚举类实例 {
//   分类1 => 返回值表达式,
//   分类2 => 返回值表达式,
//   ...
// }
