pub mod assignment;
pub mod ifthenelse;
pub mod r#return;
pub mod sequence;

pub use assignment::Assignment;
pub use ifthenelse::IfThenElse;
pub use r#return::Return;
pub use sequence::Sequence;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Assignment(assignment::Assignment),
    Expression(crate::expression::Expression),
    IfThenElse(ifthenelse::IfThenElse),
    Return(r#return::Return),
    Sequence(sequence::Sequence),
}

use crate::expression::Expression;

pub fn assignment(left: impl Into<Expression>, right: impl Into<Expression>) -> Statement {
    Statement::Assignment(Assignment {
        left: left.into(),
        right: right.into(),
    })
}

pub fn r#return(ret: Vec<impl Into<Expression>>) -> Statement {
    Statement::Return(Return {
        ret: ret.into_iter().map(|expr| expr.into()).collect(),
    })
}

pub fn sequence(sequence: Vec<Statement>) -> Statement {
    Statement::Sequence(Sequence { sequence })
}

use Statement::*;

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Assignment(s) => s.fmt(f),
            Expression(e) => e.fmt(f),
            IfThenElse(ite) => ite.fmt(f),
            Return(r) => r.fmt(f),
            Sequence(s) => s.fmt(f),
        }
    }
}

impl From<Expression> for Statement {
    fn from(e: Expression) -> Self {
        Expression(e)
    }
}
