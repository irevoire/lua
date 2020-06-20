pub mod assignment;
pub mod sequence;

use crate::{Environment, Run};
use ast::expression::Expression;
use ast::statement::Statement;

impl Run for Statement {
    fn run(&self, env: &mut Environment) -> Expression {
        match self {
            Statement::Assignment(a) => a.run(env),
            Statement::Expression(e) => e.run(env),
            Statement::Sequence(s) => s.run(env),
        }
    }
}
