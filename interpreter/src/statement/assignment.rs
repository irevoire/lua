use crate::{Environment, Run};
use ast::expression::{self, Expression};
use ast::statement::Assignment;

impl Run for Assignment {
    fn run(&self, env: &mut Environment) -> Expression {
        let right = self.right.run(env);

        if let Expression::Literal(left) = &self.left {
            env.local_insert(left.clone(), right);
        } else {
            panic!("assignment with unsupported left side");
        }

        expression::nil()
    }
}
