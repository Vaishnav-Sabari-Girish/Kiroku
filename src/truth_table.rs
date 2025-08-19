use itertools::Itertools;
use std::collections::HashSet;
use std::collections::HashMap;
use crate::eval::eval;
use crate::expr::Expr;


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
        Expr::And(a, b) | Expr::Or(a, b) => {
            collect_vars(a, set);
            collect_vars(b, set);
        },
        Expr::Xor(a, b) => {
            collect_vars(a, set);
            collect_vars(b, set);
        }

        Expr::Xnor(a, b) => {
            collect_vars(a, set);
            collect_vars(b, set);
        }
    }
}

pub fn truth_table(expr: &Expr) {
    let vars = variables(expr);
    println!("| {} | OUT |", vars.join(" | "));

    for combo in (0..vars.len()) 
        .map(|_| [false, true])
        .multi_cartesian_product()
        {
            let mut map = HashMap::new();
            for (var, val) in vars.iter().zip(combo.iter()) {
                map.insert(var.clone(), *val);
            }

            let result = eval(expr, &map);
            let inputs: Vec<String> = combo
                .iter()
                .map(|b| if *b {"1".to_string()} else {"0".to_string()})
                .collect();

            println!("| {} | {} |", inputs.join(" | "), if result { "1" } else { "0" } );
        }

}
