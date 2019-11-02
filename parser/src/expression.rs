use crate::env::Env;
use crate::function::Function;
use crate::lexreader::LexReader;
use crate::prefix_op;
use crate::statement::StatementType;
use crate::ParseError;
use lexer::Lexeme;
use lexer::Lexeme::*;

pub struct Expression {
    expr: Box<dyn ExprType>,
}

pub trait ExprType {}

impl StatementType for Expression {}

impl Expression {
    fn parse_ident(name: &str, reader: &mut LexReader, env: &mut Env) -> Result<Self, ParseError> {
        if let Some(fun) = env.functions.get_mut(name) {
            Ok(Expression {
                expr: Box::new(fun.clone().parse_call(reader, env)?),
            })
        } else if let Some(val) = env.vars.get(name) {
            let r = reader.read()?;
            match r {
                CloseParenthesis | SemiColon => {
                    reader.unread(r);
                    return Ok(Expression {
                        expr: Box::new(val.clone()),
                    });
                }
                _ => Err(format!("Unexpected {} after {}", r, name).into()),
            }
        } else {
            Err(format!("Found unexpected identifier {}", name).into())
        }
    }

    pub fn parse(lex: Lexeme, reader: &mut LexReader, env: &mut Env) -> Result<Self, ParseError> {
        match lex {
            Identifier(ident) => Self::parse_ident(&ident, reader, env),
            Lexeme::String(s) => Ok(Expression {
                expr: Box::new(crate::atomic::String::new(&s)),
            }),
            Int(i) => Ok(Expression {
                expr: Box::new(crate::atomic::Int::new(i)),
            }),
            Float(f) => Ok(Expression {
                expr: Box::new(crate::atomic::Float::new(f)),
            }),
            Minus => Ok(Expression {
                expr: Box::new(prefix_op::Minus::parse(reader, env)?),
            }),
            Plus => Ok(Expression {
                expr: Box::new(prefix_op::Plus::parse(reader, env)?),
            }),
            Length => Ok(Expression {
                expr: Box::new(prefix_op::Length::parse(reader, env)?),
            }),
            UnaryNot => Ok(Expression {
                expr: Box::new(prefix_op::Not::parse(reader, env)?),
            }),
            CloseParenthesis => {
                reader.unread(CloseParenthesis);
                Ok(Expression {
                    expr: Box::new(crate::atomic::Null::new()),
                })
            }
            OpenParenthesis => {
                let lex = reader.read()?;
                let res = Self::parse(lex, reader, env);
                match reader.read() {
                    Ok(CloseParenthesis) => res,
                    Ok(lex) => Err(format!(
                        "Expected a closing parenthesis and got {} instead",
                        lex
                    )
                    .into()),
                    _ => Err("Expected a closing parenthesis but got nothing instead".into()),
                }
            }
            e => Err(format!("Got unexpected expression {}", e).into()),
        }
    }
}
