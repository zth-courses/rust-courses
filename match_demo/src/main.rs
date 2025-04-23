// 水果枚举
#[derive(Debug)]
enum Fruit {
    Apple(Color),
    Banana,
    Orange,
}
// 青红颜色枚举
#[derive(Debug)]
enum Color {
    Red,
    Green,
}
fn witch_fruit(fruit: &Fruit) -> u8 {
    // match 表达式，类似switch case
    match fruit {
        Fruit::Apple(color) => {
            println!("apple's color is {:?}!", color);
            1
        }
        Fruit::Banana => {
            println!("banana!");
            2
        }
        Fruit::Orange => {
            println!("orange!");
            3
        }
        _ => 0, // _ =>()表示其他情况
    }
}

fn main() {
    let fruit: Fruit = Fruit::Apple(Color::Red);
    let fruit_value: u8 = witch_fruit(&fruit);

    println!("{:?}的值是{}", fruit, fruit_value);

    println!("---------------------下划线------------------------");

    // 对于不超过两种条件判断的，可以使用if let 表达式
    let fruit1 = Fruit::Banana;
    if let Fruit::Banana = fruit1 {
        println!("fruit1 is banana!");
    } else {
        println!("fruit1 is not banana!");
    }
    println!("---------------------下划线------------------------");
    // 同样的效果，使用match
    let _fruit1 = match fruit1 {
        Fruit::Banana => {
            println!("fruit1 is banana11111!");
            1
        }
        _ => {
            println!("fruit1 is not banana!");
            0
        }
    };
    println!("{:?}", _fruit1);
    println!("---------------------下划线------------------------");

    // matches!宏
    #[derive(Debug)]
    enum MyEnum {
        Foo,
        Bar,
    }
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let bool = v.iter().filter(|x: &&MyEnum| matches!(x, MyEnum::Foo));
    println!("{:?}", bool)
}
