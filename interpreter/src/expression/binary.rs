use crate::{Environment, Run};
use ast::expression::Expression;
use ast::expression::{binary, Binary, Constant};

impl Run for Binary {
    fn run(&self, env: &mut Environment) -> Expression {
        let left = match self.left.run(env) {
            Expression::Constant(Constant::Int(left)) => left,
            Expression::EarlyReturn(box Expression::Constant(Constant::Int(left))) => left,
            bad => panic!("left side of {} is not a number: {}", self.op, bad),
        };
        let right = match self.right.run(env) {
            Expression::Constant(Constant::Int(right)) => right,
            Expression::EarlyReturn(box Expression::Constant(Constant::Int(right))) => right,
            bad => panic!("right side of {} is not a number: {}", self.op, bad),
        };
        match self.op {
            binary::Operator::Add => Expression::Constant(Constant::Int(left + right)),
            binary::Operator::Sub => Expression::Constant(Constant::Int(left - right)),
            binary::Operator::Mul => Expression::Constant(Constant::Int(left * right)),
            binary::Operator::Div => Expression::Constant(Constant::Int(left / right)),
            binary::Operator::Equal => Expression::Constant(Constant::Bool(left == right)),
            binary::Operator::NotEqual => Expression::Constant(Constant::Bool(left != right)),
            binary::Operator::LowerThan => Expression::Constant(Constant::Bool(left < right)),
            binary::Operator::LowerOrEqual => Expression::Constant(Constant::Bool(left <= right)),
            binary::Operator::GreaterThan => Expression::Constant(Constant::Bool(left > right)),
            binary::Operator::GreaterOrEqual => Expression::Constant(Constant::Bool(left >= right)),
        }
    }
}
