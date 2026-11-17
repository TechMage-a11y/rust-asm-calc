use std::arch::asm;

fn validate_result(result: f64) -> Result<f64, &'static str> {
    if result.is_infinite() {
        return Err("Переполнение при вычислении!!");
    }
    if result.is_nan() {
        return Err("Некорректный математический результат (NaN)!!");
    }
    if result == 0.0 {
        return Ok(0.0);
    }
    Ok(result)
}

pub fn add(a: f64, b: f64) -> Result<f64, &'static str> {
    let result: f64;
    unsafe {
        asm!(
        "addsd {0}, {1}",
        inout(xmm_reg) a => result,
        in(xmm_reg) b,
        options(pure, nomem, nostack),
        );
    }
    validate_result(result)
}

pub fn sub(a: f64, b: f64) -> Result<f64, &'static str> {
    let result: f64;
    unsafe {
        asm!(
        "subsd {0}, {1}",
        inout(xmm_reg) a => result,
        in(xmm_reg) b,
        options(pure, nomem, nostack),
        );
    }
    validate_result(result)
}

pub fn mul(a: f64, b: f64) -> Result<f64, &'static str> {
    let result: f64;
    unsafe {
        asm!(
        "mulsd {0}, {1}",
        inout(xmm_reg) a => result,
        in(xmm_reg) b,
        options(pure, nomem, nostack),
        );
    }
    validate_result(result)
}

pub fn div(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        return Err("Деление на 0!!");
    }
    let result: f64;
    unsafe {
        asm!(
        "divsd {0}, {1}",
        inout(xmm_reg) a => result,
        in(xmm_reg) b,
        options(pure, nomem, nostack),
        );
    }
    validate_result(result)
}