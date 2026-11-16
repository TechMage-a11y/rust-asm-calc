use std::arch::asm;

pub fn add(a: i64, b: i64) -> Result<i64, &'static str> {
    let result: i64;
    let overflow: u8;
    unsafe {
        asm!(
        "add {0}, {1}",
        "seto {2}",
        inout(reg) a => result,
        in(reg) b,
        out(reg_byte) overflow,
        );
    }
    if overflow != 0 {
        return Err("Переполнение при сложении!!");
    }
    Ok(result)
}

pub fn sub(a: i64, b: i64) -> Result<i64, &'static str> {
    let result: i64;
    let overflow: u8;
    unsafe {
        asm!(
        "sub {0}, {1}",
        "seto {2}",
        inout(reg) a => result,
        in(reg) b,
        out(reg_byte) overflow,
        );
    }
    if overflow != 0 {
        return Err("Переполнение при вычитании!!")
    }
    Ok(result)
}

pub fn mul(a: i64, b: i64) -> Result<i64, &'static str> {
    let result: i64;
    let overflow: u8;
    unsafe {
        asm!(
        "imul {0}, {1}",
        "seto {2}",
        inout(reg) a => result,
        in(reg) b,
        out(reg_byte) overflow,
        );
    }
    if overflow != 0 {
        return Err("Переполнение при умножении!!")
    }
    Ok(result)
}

pub fn div(a: i64, b: i64) -> Result<i64, &'static str> {
    if b == 0 {
        return Err("Деление на 0!!");
    }
    if a == i64::MIN && b == -1 {
        return Err("Переполнение при делении!!");
    }
    let result: i64;
    unsafe {
        asm!(
        "cqo",
        "idiv {divisor}",
        inout("rax") a => result,
        out("rdx") _,
        divisor = in(reg) b,
        );
    }
    Ok(result)
}