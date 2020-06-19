use crate::env::Env;
use crate::statement::StatementType;
use crate::ParseError;
use std::io::Read;

pub struct While {}

impl StatementType for While {}

impl While {
    pub fn parse(reader: &mut impl Read, env: &mut Env) -> Result<Self, ParseError> {
        Ok(While {})
    }
}
