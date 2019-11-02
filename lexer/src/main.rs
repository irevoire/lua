use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let mut args = env::args();
    if args.len() != 2 {
        println!("usage: cargo run [filename]");
        return Ok(());
    }
    args.next(); // executable name
    let f = File::open(args.next().unwrap())?;
    let reader = BufReader::new(f);

    let lexer = lexer::Lexer::new(reader)?;
    for lexeme in lexer {
        match lexeme {
            Err(e) => {
                println!("\n\x1B[31mError: \x1B[m{}", e);
                break;
            }
            Ok(lex) => println!("\x1B[32m{:?}\x1B[m ", lex),
        }
    }
    println!("\n================================");
    println!("\nFile successfully tokenized");
    Ok(())
}
