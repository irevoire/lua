use crate::env::Env;
use nom::IResult;

#[derive(Hash, Copy, Clone)]
pub struct Value {}

impl Value {
    pub fn parse<'a>(name: &str, reader: &'a str, env: &mut Env) -> IResult<Self, &'a str> {
        Ok(Value {})
    }
}
