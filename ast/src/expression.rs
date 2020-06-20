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

pub fn binary(left: Expression, op: binary::Operator, right: Expression) -> Expression {
    Expression::Binary(Binary {
        left: Box::new(left),
        op,
        right: Box::new(right),
    })
}

pub fn constant(c: Constant) -> Expression {
    Expression::Constant(c)
}

pub fn literal<S: Into<String>>(s: S) -> Expression {
    Expression::Literal(s.into())
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
