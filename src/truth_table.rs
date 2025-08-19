use itertools::Itertools;
use std::collections::{HashSet, HashMap};
use crate::eval::eval;
use crate::expr::Expr;
use comfy_table::{Table, Row, Cell, presets::UTF8_FULL};

pub fn variables(expr: &Expr) -> Vec<String> {
    let mut set = HashSet::new();
    collect_vars(expr, &mut set);
    let mut vars: Vec<_> = set.into_iter().collect();
    vars.sort();
    vars
}

fn collect_vars(expr: &Expr, set: &mut HashSet<String>) {
    match expr {
        Expr::Var(name) => { set.insert(name.clone()); }
        Expr::Not(inner) => collect_vars(inner, set),
        Expr::And(a, b) | Expr::Or(a, b) 
        | Expr::Xor(a, b) | Expr::Xnor(a, b) 
        | Expr::Nand(a, b) | Expr::Nor(a, b) => {
            collect_vars(a, set);
            collect_vars(b, set);
        },
    }
}

pub fn truth_table(expr: &Expr) {
    let vars = variables(expr);

    // Create table with nice borders
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    // Add header row (variables + OUT)
    let mut headers: Vec<Cell> = vars.iter().map(|v| Cell::new(v)).collect();
    headers.push(Cell::new("OUT"));
    table.set_header(Row::from(headers));

    // Add rows for each input combination
    for combo in (0..vars.len()) 
        .map(|_| [false, true])
        .multi_cartesian_product()
    {
        let mut map = HashMap::new();
        for (var, val) in vars.iter().zip(combo.iter()) {
            map.insert(var.clone(), *val);
        }

        let result = eval(expr, &map);

        // Build row with input values and output
        let mut row: Vec<Cell> = combo
            .iter()
            .map(|b| Cell::new(if *b { "1" } else { "0" }))
            .collect();

        row.push(Cell::new(if result { "1" } else { "0" }));

        table.add_row(Row::from(row));
    }

    // Print the pretty table
    println!("{table}");
}
