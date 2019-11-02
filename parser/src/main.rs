use std::env;

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    if args.len() != 2 {
        println!("usage: cargo run [filename]");
        return Ok(());
    }
    args.next(); // executable name
    let reader = parser::lexreader::LexReader::from(&args.next().unwrap())?;

    let ast = parser::Ast::parse(reader);
    println!("{:?}", ast);

    Ok(())
}
