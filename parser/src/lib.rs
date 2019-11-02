mod atomic;
mod env;
mod expression;
mod function;
mod ifthenelse;
pub mod lexreader;
mod prefix_op;
mod r#return;
mod statement;
mod value;
mod r#while;

type ParseError = Box<dyn std::error::Error>;

pub struct Ast {
    vec: Vec<statement::Statement>,
    env: env::Env,
}

impl Ast {
    pub fn parse(mut reader: &mut lexreader::LexReader) -> Result<Self, ParseError> {
        let mut vec = Vec::new();
        let mut env = env::Env::base();
        loop {
            let res = statement::parse(&mut reader, &mut env);
            // exit if eof was reached
            if let Err(e) = res {
                if e.is::<lexreader::EoFError>() {
                    break;
                }
            } else {
                vec.push(res?);
            }
        }
        Ok(Ast { vec, env })
    }
}
