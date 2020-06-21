use crate::expression::Expression;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Return {
    pub ret: Vec<Expression>,
}

impl std::fmt::Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1B[31;1mreturn\x1B[m ")?;
        self.ret
            .iter()
            .map(|stmt| write!(f, "{}, ", stmt))
            .collect()
    }
}
