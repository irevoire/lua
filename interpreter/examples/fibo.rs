use ast::expression::*;
use ast::statement::*;
use interpreter::{Environment, Run};

fn main() {
    let n = std::env::args()
        .skip(1)
        .next()
        .expect("give me a number for fibo");

    let stmt = sequence(vec![
        fibo().into(),
        r#return(vec![call(
            "fibo",
            vec![constant(n.parse::<i64>().unwrap())],
        )]),
    ]);
    let mut env = Environment::new();

    println!("=================== CODE =================");
    println!("{}", stmt);
    println!("=================== EXE ==================");
    println!("{}", stmt.run(&mut env));
    println!("=================== ENV ==================");
    println!("env: {:?}", env);
}

fn fibo() -> Expression {
    let sequence = Sequence {
        sequence: vec![
            ifthenelse(
                binary(literal("n"), binary::Operator::LowerThan, constant(2)),
                Sequence::new(vec![r#return(vec![constant(1)])]),
                None,
            ),
            r#return(vec![binary(
                call(
                    "fibo",
                    vec![binary(literal("n"), binary::Operator::Sub, constant(1))],
                ),
                binary::Operator::Add,
                call(
                    "fibo",
                    vec![binary(literal("n"), binary::Operator::Sub, constant(2))],
                ),
            )]),
        ],
    };

    function(Some("fibo".into()), vec!["n"], sequence)
}
