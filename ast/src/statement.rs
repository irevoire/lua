pub mod assignment;
pub mod sequence;

pub use assignment::Assignment;
pub use sequence::Sequence;

#[derive(Clone, Debug)]
pub enum Statement {
    Sequence(sequence::Sequence),
    Assignment(assignment::Assignment),
}

use crate::expression::Expression;

pub fn sequence(sequence: Vec<Statement>) -> Statement {
    Statement::Sequence(Sequence { sequence })
}

pub fn assignment(left: Expression, right: Expression) -> Statement {
    Statement::Assignment(Assignment { left, right })
}

use Statement::*;

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sequence(s) => s.fmt(f),
            Assignment(s) => s.fmt(f),
        }
    }
}
