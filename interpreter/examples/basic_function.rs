use ast::expression::*;
use ast::statement::*;
use interpreter::Run;
use std::collections::HashMap;

fn main() {
    let stmt = sequence(vec![add(), call("add", vec![constant(1), constant(2)])]);
    let mut env = HashMap::new();

    println!("{}", stmt);
    println!("{}", stmt.run(&mut env));
    println!("env: {:?}", env);
}

fn add() -> Expression {
    let expr = binary(literal("left"), binary::Operator::Add, literal("left"));

    function(
        Some("add".into()),
        vec!["left", "right"],
        Sequence {
            sequence: vec![expr.into()],
        },
    )
}
