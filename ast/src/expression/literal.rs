#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Literal {
    pub value: String,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl From<&str> for Literal {
    fn from(s: &str) -> Self {
        Literal { value: s.into() }
    }
}
