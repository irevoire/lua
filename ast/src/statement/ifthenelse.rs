use super::Sequence;
use crate::expression::Expression;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IfThenElse {
    pub cond: Expression,
    pub if_body: Sequence,
    pub else_body: Option<Sequence>,
}

impl std::fmt::Display for IfThenElse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\x1B[33;1mif\x1B[m {} \x1B[33;1mthen\x1B[m", self.cond)?;
        self.if_body.fmt(f)?;
        if let Some(body) = &self.else_body {
            writeln!(f, "\x1B[33;1melse\x1B[m")?;
            body.fmt(f)?;
        }
        writeln!(f, "\x1B[33;1mend\x1B[m")?;
        Ok(())
    }
}
