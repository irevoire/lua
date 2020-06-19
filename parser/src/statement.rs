use crate::env::Env;
use crate::function;
/*
use crate::{
    expression::Expression, function::Function, ifthenelse::IfThenElse, r#return::Return,
    r#while::While,
};
*/
use anyhow::Result;
use nom::branch::alt;
use nom::IResult;

#[derive(Debug)]
pub enum Statement {
    Function(function::Function),
}

pub fn parse<'a>(reader: &'a str, env: &mut Env) -> IResult<Statement, &'a str> {
    alt((
        function::Function::parse_definition(reader, env),
        // Return::parse(env),
        // If::parse(env),
        // While::parse(env),
        // Expression::parse(env),
    ))(reader)
}
