mod math;

use inquire::CustomType;
use inquire::Select;
use inquire::Confirm;
use crate::math::{add, div, mul, sub};

const SEPARATOR: &'static str = "----------------";

fn main() {
    loop {
        println!("{SEPARATOR}");
        let num1 = CustomType::<i64>::new("Введите 1 число: ")
            .with_error_message("Введите целое число!!")
            .prompt()
            .unwrap_or_default();
        
        let num2 = CustomType::<i64>::new("Введите 2 число: ")
            .with_error_message("Введите целое число!!")
            .prompt()
            .unwrap_or_default();

        println!("{SEPARATOR}");
        let options = vec!["Сложение", "Вычитание", "Умножение", "Деление"];
        let select = Select::new("Выберите операцию с числами:", options)
            .prompt()
            .unwrap_or_default();

        let result = match select {
            "Сложение" => add(num1, num2),
            "Вычитание" => sub(num1, num2),
            "Умножение" => mul(num1, num2),
            "Деление" => div(num1, num2),
            _ => Err("Неизвестная операция"),
        };

        println!("{SEPARATOR}");
        match result {
            Ok(val) => println!("Результат: {}", val),
            Err(err) => println!("Ошибка: {}", err),
        }

        println!("{SEPARATOR}");
        let ans = Confirm::new("Вы хотите продолжить?")
            .with_default(false)
            .prompt();

        match ans {
            Ok(true) => {}
            Ok(false) => {
                break
            }
            Err(_) => {
                println!("Ошибка ввода");
                break
            }
        }
    }
}