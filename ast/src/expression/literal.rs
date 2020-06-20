#[derive(Debug)]
pub enum Literal {
    Int(i64),
}

use Literal::*;

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Int(i) => i.fmt(f),
        }
    }
}
