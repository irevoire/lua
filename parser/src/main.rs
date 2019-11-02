use std::env;

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    if args.len() != 2 {
        println!("usage: cargo run [filename]");
        return Ok(());
    }
    args.next(); // executable name
    let mut reader = parser::lexreader::LexReader::from(&args.next().unwrap())?;

    let ast = parser::Ast::parse(&mut reader);
    if let Err(e) = ast {
        println!("Error: {}", e);
    } else {
        println!("Was able to parse the file");
    }

    Ok(())
}
