use crate::env::Env;
/*
use crate::expression::ExprType;
use crate::statement::StatementType;
*/
use nom::IResult;

#[derive(Debug, Hash, Copy, Clone)]
pub struct Function {}

impl Function {
    pub fn new() -> Self {
        Function {}
    }

    pub fn parse_definition<'a>(
        reader: &'a str,
        env: &mut Env,
    ) -> IResult<crate::statement::Statement, &'a str> {
        unimplemented!("parse function definition");
    }

    pub fn parse_call<'a>(
        reader: &'a str,
        env: &mut Env,
    ) -> IResult<crate::statement::Statement, &'a str> {
        unimplemented!("parse function call");
    }
}
