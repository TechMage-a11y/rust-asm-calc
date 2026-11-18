use inquire::{CustomType, Select, Confirm};

const SEPARATOR: &str = "----------------------------------------";
const OPERATIONS: [&str; 4] = ["Сложение", "Вычитание", "Умножение", "Деление"];

pub enum Operation {
    Add(f64, f64),
    Sub(f64, f64),
    Mul(f64, f64),
    Div(f64, f64),
}

fn prompt_number(text: &str) -> Option<f64> {
    CustomType::<f64>::new(text)
        .with_error_message("Введите число")
        .prompt()
        .ok()
}

fn prompt_binary() -> Option<(f64, f64)> {
    let a = prompt_number("Введите 1 число: ")?;
    let b = prompt_number("Введите 2 число: ")?;
    Some((a, b))
}

pub fn get_operation() -> Option<Operation> {
    println!("{SEPARATOR}");
    let select = Select::new("Выберите операцию с числами:", Vec::from(OPERATIONS))
        .prompt()
        .ok()?;

    println!("{SEPARATOR}");
    let select_option = match select {
        "Сложение" => {
            let (a, b) = prompt_binary()?;
            Operation::Add(a, b)
        },
        "Вычитание" => {
            let (a, b) = prompt_binary()?;
            Operation::Sub(a, b)
        },
        "Умножение" => {
            let (a, b) = prompt_binary()?;
            Operation::Mul(a, b)
        },
        "Деление" => {
            let (a, b) = prompt_binary()?;
            Operation::Div(a, b)
        },
        _ => return None,
    };

    Some(select_option)
}
pub fn print_result(result: Result<f64, &'static str>) {
    println!("{SEPARATOR}");
    match result {
        Ok(val) => println!("Результат: {}", val),
        Err(err) => println!("Ошибка: {}", err),
    }
}

pub fn ask_continue() -> bool {
    println!("{SEPARATOR}");
    Confirm::new("Вы хотите продолжить?")
        .with_default(false)
        .prompt()
        .unwrap_or(false)
}