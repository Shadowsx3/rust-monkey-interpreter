use std::io::{Stdin, Stdout, Write};
use crate::lexer::Lexer;
use crate::token::TokenKind;

pub fn start(stdin: Stdin, mut stdout: Stdout) {
    loop {
        write!(stdout, ">> ").expect("Should have written prompt string >>");
        stdout.flush().expect("Should have flushed stdout");

        let mut input = String::new();
        if let Err(e) = stdin.read_line(&mut input) {
            write!(stdout, "Error: {}", e).expect("Should have written error message");
            return;;
        }

        let mut lexer = Lexer::new(input.as_str());

        loop {
            let tok = lexer.next_token();
            if tok.kind == TokenKind::Eof {
                break;
            }
            write!(stdout, "{:?}\n", tok).expect("Should have written token");
        }
    }
}