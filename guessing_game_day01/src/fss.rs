use std::fs;


pub struct Fss {}

impl Fss {
    pub fn new() -> Fss {
        return Fss {};
    }
    pub fn read(&self) {
        println!("read file");
    }
    pub fn read_file(path: &str) {
        let text = fs::read_to_string(path).unwrap();
        println!("{text}");
    }
}
