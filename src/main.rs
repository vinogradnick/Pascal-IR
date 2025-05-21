use std::{task::Context, time::Instant};

use compiler::{ir::IRContext, lexer::common::Lexer, parser::common::Parser};

pub mod compiler;
/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let buffer = std::fs::read_to_string("in.txt").unwrap();

    let lines = buffer.lines().skip(1).collect::<Vec<&str>>().join("\n");

    let start = Instant::now();
    let mut lexems = Lexer::new(&lines);

    let mut tokens = vec![];

    while !lexems.eof() {
        let item = lexems.next_token();
        tokens.push(item);
    }

    let mut parser = Parser::new(tokens);

    // Write an answer using println!("message...");
    // To debug:
    let program = parser.parse_program().unwrap();
    println!("{}", format!("{:#?}", program));
    let duration = start.elapsed();
    let mut context = IRContext::new(program, true);

    context.process();
    println!();
    println!("Время выполнения: {:?}", duration);
}
