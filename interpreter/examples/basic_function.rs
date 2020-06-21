use ast::expression::*;
use ast::statement::*;
use interpreter::{Environment, Run};

fn main() {
    let stmt = sequence(vec![
        add().into(),
        assignment(
            literal("result"),
            call("add", vec![constant(1), constant(2)]),
        ),
        r#return(vec![literal("result")]),
    ]);
    let mut env = Environment::new();

    println!("=================== CODE =================");
    println!("{}", stmt);
    println!("=================== EXE ==================");
    println!("{}", stmt.run(&mut env));
    println!("=================== ENV ==================");
    println!("env: {:?}", env);
}

fn add() -> Expression {
    let expr = binary(literal("left"), binary::Operator::Add, literal("right"));

    function(
        Some("add".into()),
        vec!["left", "right"],
        Sequence {
            sequence: vec![r#return(vec![expr])],
        },
    )
}
