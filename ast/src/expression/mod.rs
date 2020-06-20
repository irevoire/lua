pub mod binary;
pub mod literal;

#[derive(Debug)]
pub enum Expression {
    Binary(binary::Binary),
    Literal(literal::Literal),
}

use Expression::*;

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Binary(b) => b.fmt(f),
            Literal(l) => l.fmt(f),
        }
    }
}
