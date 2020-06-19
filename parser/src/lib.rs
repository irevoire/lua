// mod atomic;
mod env;
// mod expression;
mod function;
// mod ifthenelse;
// pub mod lexreader;
// mod prefix_op;
// mod r#return;
mod statement;
mod value;
// mod r#while;

use anyhow::{anyhow, Result};
use std::io::Read;

pub struct Ast {
    vec: Vec<statement::Statement>,
    env: env::Env,
}

impl Ast {
    pub fn parse(reader: &mut impl Read) -> Result<Self> {
        // let mut vec = Vec::new();
        let mut env = env::Env::base();
        let mut s = String::new();
        reader.read_to_string(&mut s).unwrap();
        let res = statement::parse(&s, &mut env);
        dbg!(res);
        /*
        loop {
            let res = statement::parse(reader, &mut env);
            // exit if eof was reached
            if let Err(e) = res {
                if e.is::<lexreader::EoFError>() {
                    break;
                } else {
                    return Err(e.into());
                }
            } else {
                vec.push(res?);
            }
        }
        Ok(Ast { vec, env })
        */
        Err(anyhow!("caca"))
    }
}
