use ast::expression::{binary, constant, literal, Constant};
use ast::statement::assignment;
use interpreter::Run;
use std::collections::HashMap;

fn main() {
    let left = constant(Constant::Int(1));
    let right = constant(Constant::Int(2));

    let expr = binary(left, binary::Operator::Add, right);

    let var = literal("a");
    let stmt = assignment(var, expr);

    let mut env = HashMap::new();

    println!("{}", stmt);
    println!("{}", stmt.run(&mut env));
    println!("env: {:?}", env);
}
