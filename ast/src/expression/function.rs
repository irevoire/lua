#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    pub name: Option<super::Literal>,
    pub params: Vec<super::Literal>,
    pub body: crate::statement::Sequence,
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\x1B[33;1mfunction\x1B[m {}(",
            self.name
                .as_ref()
                .map(|lit| &lit.value)
                .unwrap_or(&"".into())
        )?;
        for (i, param) in self.params.iter().enumerate() {
            if i == self.params.len() - 1 {
                writeln!(f, "{})", param)?;
            } else {
                write!(f, "{}, ", param)?;
            }
        }
        self.body.fmt(f)?;
        writeln!(f, "\x1B[33;1mend\x1b[m")
    }
}
