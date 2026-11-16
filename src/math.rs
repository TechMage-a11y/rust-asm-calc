use std::arch::asm;

pub fn add(a: i64, b: i64) -> Result<i64, &'static str> {
    let result: i64;
    unsafe {
        asm!(
            "add {0}, {1}",
            inout(reg) a => result,
            in(reg) b,
        );
    }
    Ok(result)
}

pub fn sub(x: i64, y: i64) -> Result<i64, &'static str> {
    let result: i64;
    unsafe {
        asm!(
        "sub {0}, {1}",
        inout(reg) x => result,
        in(reg) y,
        );
    }
    Ok(result)
}

pub fn mul(x: i64, y: i64) -> Result<i64, &'static str> {
    let result: i64;
    unsafe {
        asm!(
        "imul {0}, {1}",
        inout(reg) x => result,
        in(reg) y,
        );
    }
    Ok(result)
}

pub fn div(a: i64, b: i64) -> Result<i64, &'static str> {
    if b == 0 {
        return Err("Деление на 0!!");
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