use std::env;
use std::fs;
mod tokens;
mod lexer;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len()<2{
        std::process::exit(1);
    }

    let ruta_archivo = &args[1];

    match fs::read_to_string(ruta_archivo) {
        Ok(contenido) => {
            let mut tokens: Vec<tokens::Token>;
            tokens=lexer::tokenize(&contenido);
            
        }
        Err(error) => {
            std::process::exit(1);
        }
    }

    println!("Hello, world!");
}
