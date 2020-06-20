pub mod assignment;
pub mod sequence;

pub use assignment::Assignment;
pub use sequence::Sequence;

#[derive(Clone, Debug)]
pub enum Statement {
    Assignment(assignment::Assignment),
    Expression(crate::expression::Expression),
    Sequence(sequence::Sequence),
}

use crate::expression::Expression;

pub fn assignment(left: impl Into<Expression>, right: impl Into<Expression>) -> Statement {
    Statement::Assignment(Assignment {
        left: left.into(),
        right: right.into(),
    })
}

pub fn sequence(sequence: Vec<impl Into<Statement>>) -> Statement {
    Statement::Sequence(Sequence {
        sequence: sequence.into_iter().map(|stmt| stmt.into()).collect(),
    })
}

use Statement::*;

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Assignment(s) => s.fmt(f),
            Expression(e) => e.fmt(f),
            Sequence(s) => s.fmt(f),
        }
    }
}

impl From<Expression> for Statement {
    fn from(e: Expression) -> Self {
        Expression(e)
    }
}
