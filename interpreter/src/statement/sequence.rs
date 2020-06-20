use crate::{Environment, Run};
use ast::expression::{self, Expression};
use ast::statement::Sequence;

impl Run for Sequence {
    fn run(&self, env: &mut Environment) -> Expression {
        self.sequence.iter().for_each(|s| {
            s.run(env);
        });
        expression::nil()
    }
}
