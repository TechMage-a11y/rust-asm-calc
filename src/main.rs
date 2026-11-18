mod math;
mod cli;

use crate::math::{add, div, mul, sub};
use crate::cli::{get_operation, Operation, print_result, ask_continue};

fn main() {
    loop {
        let Some(operation) = get_operation() else {
            break
        };

        let result = match operation {
            Operation::Add(a, b) => add(a, b),
            Operation::Sub(a, b) => sub(a, b),
            Operation::Div(a, b) => div(a, b),
            Operation::Mul(a, b) => mul(a, b),
        };

        print_result(result);

        if !ask_continue() {
            break
        }
    }
}