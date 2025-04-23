fn main() {
    println!("Hello, world!");
    // if条件语句 条件表达式 number > 3不需要用小括号包括（注意，不需要不是不允许）
    // 且条件必须是bool类型
    let number = 3;
    if number > 3 {
        println!("condition was true");
    }  else if number == 3 {
        println!("condition was equal");
    }  else {
        println!("condition was false");
    }
    // if条件语句可以返回值, 实现类似三元运算符的功能
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    
    // if条件语句可以返回值，但是返回值类型必须一致, 否则编译报错
    // `if` and `else` have incompatible types
    // let condition = true;
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {}", number);

}
