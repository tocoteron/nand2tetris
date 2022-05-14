mod parser;
use parser::parse_program;

mod code;
use code::generate_code;

use std::io::{self, BufRead, Error};

fn main() -> io::Result<()> {
    let program = read_program().unwrap();
    let ast = parse_program(&program);
    let code = generate_code(&ast);

    write_code(code);

    Ok(())
}

fn read_program() -> Result<Vec<String>, Error> {
    io::stdin().lock().lines().collect()
}

fn write_code(code: Vec<String>) {
    code.iter().for_each(|line| println!("{}", line))
}