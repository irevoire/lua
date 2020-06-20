#[derive(Clone, Debug)]
pub struct Call {
    pub name: super::Literal,
    pub params: Vec<super::Expression>,
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.name)?;
        for (i, param) in self.params.iter().enumerate() {
            if i == self.params.len() - 1 {
                write!(f, "{}", param)?;
            } else {
                write!(f, "{}, ", param)?;
            }
        }
        write!(f, ")")
    }
}
