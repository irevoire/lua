use crate::{Environment, Run};
use ast::expression::{self, Expression};
use ast::statement::Return;

impl Run for Return {
    fn run(&self, env: &mut Environment) -> Expression {
        self.ret
            .iter()
            .next()
            .unwrap_or(&expression::nil())
            .run(env)
    }
}
