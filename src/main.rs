#![allow(dead_code)]
#![allow(unused_variables)]
use std::{io::stdin, time::Instant};

use compiler::{ir::IRContext, lexer::common::Lexer, parser::common::Parser};

pub mod compiler;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}
/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

const RUNTIME_CODEINGAME: bool = true;

fn main() {
    let buffer = if RUNTIME_CODEINGAME == false {
        let buffer = std::fs::read_to_string("in.txt").unwrap();

        buffer.lines().skip(1).collect::<Vec<&str>>().join("\n")
    } else {
        let mut input_line = String::new();
        stdin().read_line(&mut input_line).unwrap();
        let n = parse_input!(input_line, i32);
        let mut source = String::new();
        for i in 0..n as usize {
            let mut input_line = String::new();
            stdin().read_line(&mut input_line).unwrap();

            source.push_str(&input_line);
        }
        source
    };

    let start = Instant::now();
    let mut lexems = Lexer::new(&buffer);

    let mut tokens = vec![];

    while !lexems.eof() {
        let item = lexems.next_token();
        tokens.push(item);
    }
    eprintln!("{:#?}", &tokens);

    let mut parser = Parser::new(tokens);

    // Write an answer using println!("message...");
    // To debug:
    let program = parser.parse_program();

    let node = match program {
        Ok(node) => node,
        Err(err) => {
            println!("{}", err.msg());
            return;
        }
    };

    eprintln!("{}", format!("{:#?}", node));
    let duration = start.elapsed();
    let mut context = IRContext::new(node, true);

    context.process();
    eprintln!();
    eprintln!("Время выполнения: {:?}", duration);
}
