use crate::env::Env;
use crate::lexreader::LexReader;
use crate::statement::StatementType;
use crate::ParseError;

pub struct While {}

impl StatementType for While {}

impl While {
    pub fn parse(reader: &mut LexReader, env: &mut Env) -> Result<Self, ParseError> {
        Ok(While {})
    }
}
