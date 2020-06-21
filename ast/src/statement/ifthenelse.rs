use super::{Sequence, Statement};
use crate::expression::Expression;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IfThenElse {
    pub cond: Expression,
    pub if_body: Sequence,
    pub else_body: Option<Sequence>,
}

impl std::fmt::Display for IfThenElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "if {} then", self.cond)?;
        self.if_body.fmt(f)?;
        if let Some(body) = &self.else_body {
            writeln!(f, "else")?;
            body.fmt(f)
        } else {
            Ok(())
        }
    }
}
