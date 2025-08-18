use crate::expr::Expr;
use std::collections::HashMap;

pub fn eval(expr: &Expr, vars : &HashMap<String, bool>) -> bool {
    match expr {
        Expr::Var(name) => *vars.get(name).unwrap_or(&false),
        Expr::Not(inner) => !eval(inner, vars),
        Expr::And(a, b) => eval(a, vars) && eval(b, vars),
        Expr::Or(a, b) => eval(a, vars) || eval(b, vars),
    }
}
