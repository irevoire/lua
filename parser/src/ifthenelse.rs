use crate::env::Env;
use crate::statement::StatementType;
use crate::ParseError;
use std::io::Read;

pub struct IfThenElse {}

impl StatementType for IfThenElse {}

impl IfThenElse {
    pub fn parse(reader: &mut impl Read, env: &mut Env) -> Result<Self, ParseError> {
        Ok(IfThenElse {})
    }
}
