fn main() {
    // Create a new vector, 元素类型必须相同
    let mut v = Vec::new();
    // Add some elements
    v.push(1);
    v.push(2);
    v.push(3);
    // Print the vector
    println!("{:?}", v);
    // Get the first element
    let first = v[0];
    // Get the second element, get()方式更安全
    let second = v.get(1).unwrap();

    // Print the first and second elements
    println!("First: {}, Second: {}", first, second);

    // 使用from()方法创建一个新的vector
    let vec2 = Vec::from([1, 2, 3, 4]);
    println!("{:?}", vec2);

    // 使用vec!宏创建一个新的vector
    let mut v = vec![1, 2, 3];
    v.push(4);
    // Print the vector
    println!("{:?}", v);

    //  创建一个枚举类型的vector，枚举类型的元素可以是不同的类型
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}的第一个值{:?}", row, row[0]);

    // 元组vector
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];
    println!("元组vector：{:?}, {:?}", teams_list, teams_list[0]);
}
