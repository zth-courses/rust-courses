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
  //  生成一个正方形
  fn square (size: u32) -> Rectangle {
    Rectangle{
      width: size,
      height: size,
    }
  }
}