use crate::env::Env;
use crate::statement::StatementType;
use crate::ParseError;
use std::io::Read;

pub struct Return {}
impl StatementType for Return {}

impl Return {
    pub fn parse(reader: &mut impl Read, env: &mut Env) -> Result<Self, ParseError> {
        Ok(Return {})
    }
}
