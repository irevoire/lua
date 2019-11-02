use crate::env::Env;
use crate::expression::ExprType;
use crate::lexreader::LexReader;
use crate::statement::StatementType;
use crate::ParseError;

pub struct Length {}

impl ExprType for Length {}

impl Length {
    pub fn parse(reader: &mut LexReader, env: &mut Env) -> Result<Self, ParseError> {
        Err("Unimplemented".into())
    }
}
