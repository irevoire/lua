use crate::env::Env;
use crate::expression::ExprType;
use crate::lexreader::LexReader;
use crate::ParseError;

pub struct Null {}

impl ExprType for Null {}

impl Null {
    pub fn parse(reader: &mut LexReader, env: &mut Env) -> Result<Self, ParseError> {
        Err("Unimplemented".into())
    }
    pub fn new() -> Self {
        Null {}
    }
}
