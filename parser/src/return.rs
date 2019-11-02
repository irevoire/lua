use crate::env::Env;
use crate::lexreader::LexReader;
use crate::statement::StatementType;
use crate::ParseError;

// #[derive(Statement)]

pub struct Return {}
impl StatementType for Return {}

impl Return {
    pub fn parse(reader: &mut LexReader, env: &mut Env) -> Result<Self, ParseError> {
        Ok(Return {})
    }
}
