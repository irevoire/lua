pub mod operator;
pub use operator::Operator;

use super::Expression;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Binary {
    pub left: Box<Expression>,
    pub op: Operator,
    pub right: Box<Expression>,
}

impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.op, self.right)
    }
}
