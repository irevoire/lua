#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    LowerThan,
    LowerOrEqual,
    GreaterThan,
    GreaterOrEqual,
}

use Operator::*;

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Equal => write!(f, "="),
            NotEqual => write!(f, "!="),
            LowerThan => write!(f, "<"),
            LowerOrEqual => write!(f, "<="),
            GreaterThan => write!(f, ">"),
            GreaterOrEqual => write!(f, ">="),
        }
    }
}
