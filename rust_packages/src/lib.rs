mod back_of_house {
    // 枚举需要明确申明pub，否则外部无法访问
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
        pub fn spring(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("strawberries"),
            }
        }
    }
    // 注意公共（pub）枚举里的变体都是公共的
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // Absolute path
    let order1 = crate::back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}