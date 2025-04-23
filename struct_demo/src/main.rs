#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
  // 参数有selfshi 是结构体关联方法，没有self参数是结构体关联函数
  //  计算面积
  fn area(&self) -> u32 {
    self.width * self.height
  }
  //  判断是否能够容纳另一个矩形
  fn can_hold(&self, other: &Rectangle) ->bool {
    return self.width > other.width && self.height > other.height;
  }
  // 没有self参数是结构体关联函数
  //  生成一个正方形
  fn square (size: u32) -> Rectangle {
    Rectangle{
      width: size,
      height: size,
    }
  }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let _num = &rect1.width;

    println!("rect1的width是：{}， {}", _num, rect1.width);

  // 结构体关联函数调用方式
   let rect3 = Rectangle::square(10);
   
   let ares1: u32 = rect1.area();

   println!("rect1的面积是：{}", ares1);
   
   let is_contains: bool = rect2.can_hold(&rect1);

   println!("rect2是否能够容纳rect1：{}", is_contains);

   println!("rect3是：{:#?}", rect3)
}