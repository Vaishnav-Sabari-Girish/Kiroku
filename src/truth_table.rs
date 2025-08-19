use itertools::Itertools;
use std::collections::{HashSet, HashMap};
use crate::eval::eval;
use crate::expr::Expr;
use comfy_table::{
    Table,
    presets::UTF8_FULL,
    modifiers::UTF8_ROUND_CORNERS,
    ContentArrangement
};

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

pub fn truth_table(expr: &Expr) -> String {
    let vars = variables(expr);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    let mut header = vars.clone();
    header.push("OUT".to_string());
    table.set_header(header);

    for combo in (0..vars.len())
        .map(|_| [false, true])
        .multi_cartesian_product()
        {
            let mut map = HashMap::new();
            for (var, val) in vars.iter().zip(combo.iter()) {
                map.insert(var.clone(), *val);
            }

            let result = eval(expr, &map);

            let mut row: Vec<String> = combo
                .iter()
                .map(|b| if *b { "1".to_string() } else { "0".to_string() })
                .collect();

            row.push(if result { "1".to_string() } else { "0".to_string() });
            table.add_row(row);
        }
    table.to_string()
}
