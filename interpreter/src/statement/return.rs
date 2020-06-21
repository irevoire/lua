use crate::{Environment, Run};
use ast::expression::{self, early_return, Expression};
use ast::statement::Return;

impl Run for Return {
    fn run(&self, env: &mut Environment) -> Expression {
        let res = self
            .ret
            .iter()
            .next()
            .unwrap_or(&expression::nil())
            .run(env);
        early_return(res)
    }
}
