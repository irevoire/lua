use crate::expression::Expression;

#[derive(Clone, Debug)]
pub struct Assignment {
    pub left: Expression,
    pub right: Expression,
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} = {}", self.left, self.right)
    }
}
