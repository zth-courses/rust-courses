use std::io;
use std::io::Write;

fn main() {
    let calculator = SimpleCalculator;
    let mut user_input: UserInput<SimpleCalculator> = UserInput::new(calculator);

    loop {
        match user_input.computed() {
            Ok(result) => println!("计算结果: {}", result),
            Err(e) => println!("错误: {}", e),
        }
    }
}

trait Calculator {
    fn add(&self, a: f64, b: f64) -> f64;
    fn subtract(&self, a: f64, b: f64) -> f64;
    fn multiply(&self, a: f64, b: f64) -> f64;
    fn divide(&self, a: f64, b: f64) -> Result<f64, String>;
}

struct SimpleCalculator;

impl Calculator for SimpleCalculator {
    fn add(&self, a: f64, b: f64) -> f64 {
        a + b
    }

    fn subtract(&self, a: f64, b: f64) -> f64 {
        a - b
    }

    fn multiply(&self, a: f64, b: f64) -> f64 {
        a * b
    }

    fn divide(&self, a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            return Err("除数不能为零！".to_string());
        }
        Ok(a / b)
    }
}

struct UserInput<T: Calculator> {
    calculator: T,
    expr: String,
}

impl<T: Calculator> UserInput<T> {
    fn new(calculator: T) -> Self {
        Self {
            calculator,
            expr: String::new(),
        }
    }

    fn computed(&mut self) -> Result<f64, String> {
        self.expr.clear();
        print!("请输入表达式（例如：1.5 + 2.3）: ");
        io::stdout().flush().map_err(|e| e.to_string())?;
        io::stdin().read_line(&mut self.expr).map_err(|e| e.to_string())?;

        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut op: Option<char> = None;
        let mut has_decimal_point = false;

        for chr in self.expr.trim().chars() {
            if chr.is_digit(10) || (chr == '.' && !has_decimal_point) {
                if chr == '.' {
                    has_decimal_point = true;
                }
                if op.is_none() {
                    // 运算符为空，将当前字符添加到num1中
                    num1.push(chr);
                } else {
                    num2.push(chr);
                }
                continue;
            }

            if chr == '.' && has_decimal_point {
                return Err("数字中不能有多个小数点！".to_string());
            }

            match chr {
                '+' | '-' | '*' | '/' if op.is_none() => {
                    op = Some(chr);
                    has_decimal_point = false;  // 重置小数点标志，为第二个数字做准备
                }
                _ if chr.is_whitespace() => continue,
                _ => return Err(format!("表达式格式无效, 出现无效的字符: {}", chr)),
            }
        }

        if num1.is_empty() || num2.is_empty() || op.is_none() {
            return Err(format!("表达式格式无效: {}", self.expr));
        }

        let num1 = num1.parse::<f64>().map_err(|_| format!("第一个数字格式无效: {}", num1))?;
        let num2 = num2.parse::<f64>().map_err(|_| format!("第二个数字格式无效: {}", num2))?;
        let op = op.unwrap();

        match op {
            '+' => Ok(self.calculator.add(num1, num2)),
            '-' => Ok(self.calculator.subtract(num1, num2)),
            '*' => Ok(self.calculator.multiply(num1, num2)),
            '/' => self.calculator.divide(num1, num2),
            _ => unreachable!(),
        }
    }
}
