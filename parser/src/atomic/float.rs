use crate::env::Env;
use crate::expression::ExprType;
use crate::lexreader::LexReader;
use crate::ParseError;

pub struct Float {
    f: f64,
}

impl ExprType for Float {}

impl Float {
    pub fn parse(reader: &mut LexReader, env: &mut Env) -> Result<Self, ParseError> {
        Err("Unimplemented".into())
    }

    pub fn new(f: f64) -> Self {
        Self { f }
    }
}
