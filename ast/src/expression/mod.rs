pub mod binary;
pub mod constant;

pub use binary::Binary;
pub use constant::Constant;

#[derive(Clone, Debug)]
pub enum Expression {
    Binary(binary::Binary),
    Constant(constant::Constant),
    Literal(String),
}

use Expression::*;

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Binary(b) => b.fmt(f),
            Constant(c) => c.fmt(f),
            Literal(s) => s.fmt(f),
        }
    }
}
