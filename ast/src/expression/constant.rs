#[derive(Clone, Debug)]
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
