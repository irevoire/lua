use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let mut args = env::args();
    if args.len() != 2 {
        println!("usage: cargo run [filename]");
        return Ok(());
    }
    args.next(); // executable name
    println!("open file");
    let mut f = File::open(args.next().unwrap())?;
    let mut reader = BufReader::new(f);

    println!("create lexer");
    let mut lexer = lexer::Lexer::new(reader)?;
    println!("loop on lexemes");
    for lexeme in lexer {
        match lexeme {
            Err(e) => {
                println!("\n\x1B[31mError \x1B[m");
                break;
            }
            Ok(lex) => println!("\x1B[32m{:?}\x1B[m ", lex),
        }
    }
    println!("\n================================");
    println!("\nFile successfully tokenized");
    Ok(())
}
