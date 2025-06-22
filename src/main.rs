// src/main.rs
// NovaLang ana giriş noktası: REPL ve dosya çalıştırıcı

mod token;
mod lexer;
mod parser;
mod ast;
mod semantic;
mod vm;
mod runtime;
mod error;

use std::env;
use std::fs;
use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use crate::vm::VM;
use crate::error::NovaError;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // Dosya çalıştırıcı
        let filename = &args[1];
        let code = fs::read_to_string(filename).expect("Dosya okunamadı");
        if let Err(e) = run_code(&code) {
            e.display();
        }
    } else {
        // REPL
        repl();
    }
}

fn run_code(code: &str) -> Result<(), NovaError> {
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        if let crate::token::TokenKind::Eof = tok.kind {
            break;
        }
        tokens.push(tok);
    }
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program)?;
    let mut vm = VM::new();
    vm.run(&program)?;
    Ok(())
}

fn repl() {
    println!("NovaLang REPL'e hoş geldiniz! (Çıkmak için 'exit')");
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        if input.trim() == "exit" {
            break;
        }
        if let Err(e) = run_code(&input) {
            e.display();
        }
    }
} 