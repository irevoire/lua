use crate::env::Env;
use crate::expression::ExprType;
use crate::lexreader::LexReader;
use crate::ParseError;

pub struct String {
    s: std::string::String,
}

impl ExprType for String {}

impl String {
    pub fn parse(reader: &mut LexReader, env: &mut Env) -> Result<Self, ParseError> {
        Err("Unimplemented".into())
    }

    pub fn new(s: &str) -> Self {
        Self {
            s: std::string::String::from(s),
        }
    }
}
