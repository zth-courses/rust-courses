use std::collections::HashMap;

fn main() {
    // hahMap K and V must be same type
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // 访问hashmap, 返回Option<&V>
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(v) => println!("{}: {}", team_name, v),
        None => println!("{}: {}", team_name, 0),
    }
    println!("{:?}", score);

    // 遍历hashmap
    for (key, value) in scores.iter() {
        println!("遍历{}: {}", key, value);
    }

    // 更新hashmap
    //  1. 覆盖
    scores.insert(String::from("Blue"), 25);

    // 2. 仅当key不存在时插入
    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);

    // 3. 根据旧值更新
    let text: &str = "hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // vec -> hashmap
    let teams: Vec<String> = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores: Vec<i32> = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("vec -> hashmap: {:?}", scores);

    // vec -> hashmap
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];
    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
    println!("vec -> hashmap: {:?}", teams_map);
}
