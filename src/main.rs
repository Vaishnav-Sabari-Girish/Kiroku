mod expr;
mod parser;
mod eval;
mod truth_table;

use parser::parse_expr;
use truth_table::truth_table;
use std::io;

fn main() {
    println!("Enter the Expression : ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read Input");
    println!();
    println!("Expression: {}", input);
    println!();

    let expr = parse_expr(input.trim());
    truth_table(&expr);
}
