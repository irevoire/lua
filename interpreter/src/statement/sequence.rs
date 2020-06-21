use crate::{Environment, Run};
use ast::expression::{self, Expression};
use ast::statement::Sequence;

impl Run for Sequence {
    fn run(&self, env: &mut Environment) -> Expression {
        for stmt in &self.sequence {
            match stmt.run(env) {
                ret @ Expression::EarlyReturn(_) => return ret,
                expr => expr,
            };
        }
        expression::nil()
    }
}
