use crate::env::Env;
use crate::expression::ExprType;
use crate::lexreader::LexReader;
use crate::statement::StatementType;
use crate::ParseError;

#[derive(Hash, Copy, Clone)]
pub struct Function {}

impl StatementType for Function {}
impl ExprType for Function {}

impl Function {
    pub fn new() -> Self {
        Function {}
    }

    pub fn parse_definition(reader: &mut LexReader, env: &mut Env) -> Result<Self, ParseError> {
        Err("Unimplemented".into())
    }

    pub fn parse_call(
        &mut self,
        reader: &mut LexReader,
        env: &mut Env,
    ) -> Result<Self, ParseError> {
        Err("Unimplemented parse call".into())
    }
}
