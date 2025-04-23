use dialoguer::{Confirm, Input, Password, Select};

fn main() {
    let name: String = Input::new()
        .with_prompt("请输入您的姓名")
        .allow_empty(false)
        .interact()
        .unwrap();

    let password: String = Password::new()
        .with_prompt("请输入您的密码")
        .interact()
        .unwrap();

    let gender_options = vec!["男", "女", "其他"];
    let gender = Select::new()
        .items(&gender_options)
        .with_prompt("请选择您的性别")
        .interact()
        .unwrap();

    println!("您输入的姓名是: {}", name);
    println!("您输入的密码是: {}", password);
    println!("您选择的性别是: {}", gender);

    let confirmed = Confirm::new()
        .with_prompt("您确定要提交吗?")
        .interact()
        .unwrap();

    if confirmed {
        println!("已确认提交");
    } else {
        println!("取消提交");
    }
}