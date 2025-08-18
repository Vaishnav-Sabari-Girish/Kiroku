mod expr;
mod parser;
mod eval;
mod truth_table;

use parser::parse_expr;
use truth_table::truth_table;

fn main() {
    let input = "((A * B) | (!C * !(A + B)))";
    println!();
    println!("Expression: {}", input);
    println!();

    let expr = parse_expr(input);
    truth_table(&expr);
}
