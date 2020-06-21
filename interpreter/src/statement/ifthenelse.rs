use crate::{Environment, Run};
use ast::expression::{self, Expression};
use ast::statement::{IfThenElse, Statement};

impl Run for IfThenElse {
    fn run(&self, env: &mut Environment) -> Expression {
        let cond = self.cond.run(env);
        if cond != Expression::Nil {
            self.if_body.run(env)
        } else if let Some(body) = &self.else_body {
            body.run(env)
        } else {
            expression::nil()
        }
    }
}
