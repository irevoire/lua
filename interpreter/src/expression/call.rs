use crate::{Environment, Run};
use ast::expression::{Call, Expression};
use ast::statement::assignment;

impl Run for Call {
    fn run(&self, env: &mut Environment) -> Expression {
        let fun = match env.get(&self.name) {
            Some(Expression::Function(f)) => f.clone(),
            Some(_) => panic!("{} is not a function, thus it can not be called", self.name),
            None => panic!("Unknown function {}", self.name),
        };
        if fun.params.len() != self.params.len() {
            panic!("bad number of parameters");
        }

        for (name, expr) in fun.params.iter().zip(&self.params) {
            assignment(name.clone(), expr.run(env)).run(env);
        }
        fun.body.run(env)
    }
}
