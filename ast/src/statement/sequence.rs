use super::Statement;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sequence {
    pub sequence: Vec<Statement>,
}

impl std::fmt::Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.sequence
            .iter()
            .map(|stmt| writeln!(f, "{}", stmt))
            .collect()
    }
}
