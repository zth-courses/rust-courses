fn main() {
    //  if let 分支
    // while let 分支
    let mut stack = Vec::new();
    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    for x in &stack {
        println!("{}", x);
    }
    // 只会匹配一次
    if let Some(x) = stack.pop() {
        println!("{}", x);
    }
    // 只要模式匹配就一直进行 while 循环
    while let Some(x) = stack.pop() {
        println!("{}", x);
    }
  
}
