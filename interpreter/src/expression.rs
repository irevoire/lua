pub mod binary;

use crate::{Environment, Run};
use ast::expression::Expression;

use Expression::*;

impl Run for Expression {
    fn run(&self, env: &mut Environment) -> Expression {
        match self {
            Binary(b) => b.run(env),
            expr => expr.clone(), // nothing to execute
        }
    }
}
