use crate::{Environment, Run};
use ast::expression::{self, Expression, Function};
use ast::statement::assignment;

impl Run for Function {
    fn run(&self, env: &mut Environment) -> Expression {
        let mut fun = self.clone();
        if let Some(name) = fun.name.take() {
            assignment(name.clone(), fun).run(env);
            expression::nil()
        } else {
            fun.into()
        }
    }
}
