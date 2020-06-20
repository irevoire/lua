use ast::expression::{binary, Constant, Expression};
use interpreter::Run;
use std::collections::HashMap;

fn main() {
    let left = Expression::Constant(Constant::Int(1));
    let right = Expression::Constant(Constant::Int(2));

    let expr = Expression::Binary(binary::Binary {
        left: Box::new(left),
        op: binary::Operator::Add,
        right: Box::new(right),
    });

    let mut env = HashMap::new();

    println!("{}", expr);
    println!("{}", expr.run(&mut env));
}
