use std::arch::asm;

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
    if result.is_infinite() {
        return Err("Переполнение при сложении!!");
    }
    Ok(result)
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
    if result.is_infinite() {
        return Err("Переполнение при вычитании!!");
    }
    Ok(result)
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
    if result.is_infinite() {
        return Err("Переполнение при умножении!!");
    }
    Ok(result)
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
    if result.is_infinite() {
        return Err("Переполнение при делении!!");
    }
    Ok(result)
}