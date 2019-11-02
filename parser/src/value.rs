use crate::env::Env;
use crate::expression::ExprType;
use crate::lexreader::LexReader;
use crate::ParseError;

#[derive(Hash, Copy, Clone)]
pub struct Value {}

impl ExprType for Value {}

impl Value {
    pub fn parse(name: &str, reader: &mut LexReader, env: &mut Env) -> Result<Self, ParseError> {
        Ok(Value {})
    }
}
