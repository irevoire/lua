use crate::env::Env;
use crate::expression::ExprType;
use crate::lexreader::LexReader;
use crate::statement::StatementType;
use crate::ParseError;

pub struct Not {}

impl ExprType for Not {}

impl Not {
    pub fn parse(reader: &mut LexReader, env: &mut Env) -> Result<Self, ParseError> {
        Err("Unimplemented".into())
    }
}
