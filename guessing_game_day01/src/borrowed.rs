/// 所有权 租借
pub struct Borr {}

impl Borr {
    pub fn init() {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // 复制一份s1
        let s3: &String = &s1; // 租借(引用)s1
        let s4: String = s2; // s2 失效
        println!("s1 is {}, s3 is {}, s4 is {}", s1, s3, s4);
    }
}
