use crate::env::Env;
use crate::lexreader::LexReader;
use crate::ParseError;
use crate::{
    expression::Expression, function::Function, ifthenelse::IfThenElse, r#return::Return,
    r#while::While,
};
use lexer::Lexeme::*;

pub trait StatementType {}

pub struct Statement {
    stmt: Box<dyn StatementType>,
}

pub fn parse(reader: &mut LexReader, env: &mut Env) -> Result<Statement, ParseError> {
    let lex = reader.read()?;
    let stmt: Box<dyn StatementType> = match lex {
        Function => Box::new(Function::parse_definition(reader, env)?),
        Return => Box::new(Return::parse(reader, env)?),
        If => Box::new(IfThenElse::parse(reader, env)?),
        While => Box::new(While::parse(reader, env)?),
        lex => Box::new(Expression::parse(lex, reader, env)?),
        /*
        err => {
            return Err(format!(
                "Got {} while Identifier, Return, If or While were expected",
                err
            )
            .into())
        }
        =*/
    };
    Ok(Statement { stmt })
}
