#[derive(Debug, Clone)]
pub enum Expr {
    Var(String),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Xnor(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Nand(Box<Expr>, Box<Expr>),
    Nor(Box<Expr>, Box<Expr>),
}
