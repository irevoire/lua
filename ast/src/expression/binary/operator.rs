#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Add,
    Sub,
}

use Operator::*;

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
        }
    }
}
