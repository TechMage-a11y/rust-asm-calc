use inquire::{CustomType, Select, Confirm};

const SEPARATOR: &str = "----------------------------------------";

pub enum Operation {
    Add(f64, f64),
    Sub(f64, f64),
    Mul(f64, f64),
    Div(f64, f64),
}

fn prompt_number(text: &str) -> f64 {
    CustomType::<f64>::new(text)
        .with_error_message("Введите число")
        .prompt()
        .unwrap_or_default()
}

pub fn get_operation() -> Option<Operation> {
    println!("{SEPARATOR}");

    let options = vec!["Сложение", "Вычитание", "Умножение", "Деление"];
    let select = Select::new("Выберите операцию с числами:", options)
        .prompt()
        .ok()?;

    println!("{SEPARATOR}");
    let select_option = match select {
        "Сложение" => Operation::Add(
            prompt_number("Введите 1 число: "),
            prompt_number("Введите 2 число: "),
        ),
        "Вычитание" => Operation::Sub(
            prompt_number("Введите 1 число: "),
            prompt_number("Введите 2 число: "),
        ),
        "Умножение" => Operation::Mul(
            prompt_number("Введите 1 число: "),
            prompt_number("Введите 2 число: "),
        ),
        "Деление" => Operation::Div(
            prompt_number("Введите 1 число: "),
            prompt_number("Введите 2 число: "),
        ),
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