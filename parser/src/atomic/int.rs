use crate::env::Env;
use crate::expression::ExprType;
use crate::lexreader::LexReader;
use crate::ParseError;

pub struct Int {
    i: i64,
}

impl ExprType for Int {}

impl Int {
    pub fn parse(reader: &mut LexReader, env: &mut Env) -> Result<Self, ParseError> {
        Err("Unimplemented".into())
    }

    pub fn new(i: i64) -> Self {
        Self { i }
    }
}
