use ast::expression::*;
use ast::statement::*;
use interpreter::Run;
use std::collections::HashMap;

fn main() {
    let stmt = sequence(vec![
        add().into(),
        assignment(
            literal("result"),
            call("add", vec![constant(1), constant(2)]),
        ),
        r#return(vec![literal("result")]),
    ]);
    let mut env = HashMap::new();

    println!("=================== CODE =================");
    println!("{}", stmt);
    println!("=================== EXE ==================");
    println!("{}", stmt.run(&mut env));
    println!("=================== ENV ==================");
    println!("env: {:?}", env);
}

fn add() -> Expression {
    let expr = binary(literal("left"), binary::Operator::Add, literal("left"));

    function(
        Some("add".into()),
        vec!["left", "right"],
        Sequence {
            sequence: vec![r#return(vec![expr])],
        },
    )
}
