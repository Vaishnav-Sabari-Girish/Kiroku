use pest::Parser;
use pest_derive::Parser;

use crate::expr::Expr;

#[derive(Parser)]
#[grammar = "boolean.pest"]
pub struct BooleanParser;

pub fn parse_expr(input: &str) -> Expr {
    let pairs = BooleanParser::parse(Rule::expr, input)
        .expect("Parse Error")
        .next()
        .unwrap();

    build_ast(pairs)
}

fn build_ast(pair: pest::iterators::Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::ident => Expr::Var(pair.as_str().to_string()),
        Rule::not => {
            let text = pair.as_str().to_string();
            let mut inner = pair.into_inner();
            let mut expr = build_ast(inner.next().unwrap());
            //Apply NOT operators if present 
            for _ in text.chars().filter(|&c| c == '!' || c == '\'') {
                expr = Expr::Not(Box::new(expr));
            }
            expr
        }
        Rule::and => {
            let mut inner = pair.into_inner();
            let first = build_ast(inner.next().unwrap());
            inner.fold(first, |left, next| Expr::And(Box::new(left), Box::new(build_ast(next))))
        }
        Rule::or => {
            let mut inner = pair.into_inner();
            let first = build_ast(inner.next().unwrap());
            inner.fold(first, |left, next| Expr::Or(Box::new(left), Box::new(build_ast(next))))
        }
        Rule::xor => {
            let mut inner = pair.into_inner();
            let first = build_ast(inner.next().unwrap());
            inner.fold(first, |left, next| Expr::Xor(Box::new(left), Box::new(build_ast(next))))
        }
        Rule::expr | Rule::primary => build_ast(pair.into_inner().next().unwrap()),
        _ => unreachable!(),
    }
}
