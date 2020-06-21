#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Constant {
    Int(i64),
    String(std::string::String),
}

use Constant::*;

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Int(i) => i.fmt(f),
            String(s) => s.fmt(f),
        }
    }
}

impl From<i64> for Constant {
    fn from(i: i64) -> Self {
        Constant::Int(i)
    }
}

impl From<std::string::String> for Constant {
    fn from(s: std::string::String) -> Self {
        Constant::String(s)
    }
}

impl From<&str> for Constant {
    fn from(s: &str) -> Self {
        Constant::String(s.into())
    }
}
