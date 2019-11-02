use crate::env::Env;
use crate::lexreader::LexReader;
use crate::statement::StatementType;
use crate::ParseError;

pub struct IfThenElse {}

impl StatementType for IfThenElse {}

impl IfThenElse {
    pub fn parse(reader: &mut LexReader, env: &mut Env) -> Result<Self, ParseError> {
        Ok(IfThenElse {})
    }
}
