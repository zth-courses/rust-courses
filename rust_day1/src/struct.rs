/*
 * @Author: 阮志雄
 * @Date: 2022-10-15 14:19:14
 * @LastEditTime: 2022-10-15 14:21:37
 * @LastEditors: 阮志雄
 * @Description: In User Settings Edit
 * @FilePath: \rust_demo\src\struct.rs
 */
struct Dog {
  name: String,
  age: i8
}

fn main() {
  let my_dog = Dog {
    name: String::from("paopao"),
    age: 3,
  };
  let str = mydog.name.clone();
  println!("str={}", str);
  println!("mydog: name={},age={}", mydog.name, mydog.age);
}