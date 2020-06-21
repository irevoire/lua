pub mod binary;
pub mod call;
pub mod function;

use crate::{Environment, Run};
use ast::expression::Expression;

use Expression::*;

impl Run for Expression {
    fn run(&self, env: &mut Environment) -> Expression {
        match self {
            Binary(b) => b.run(env),
            Call(c) => c.run(env),
            Function(f) => f.run(env),
            Literal(l) => env
                .get(l)
                .expect(&format!(
                    "trying to acces to the literal {} which is not defined in the current scope",
                    l
                ))
                .clone(),
            Constant(_) | EarlyReturn(_) | Nil => self.clone(), // nothing to execute
        }
    }
}
