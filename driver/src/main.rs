use std::env;
use std::fs;
use std::process::exit;

use frontend::*;
use common::*;

#[derive(Debug, PartialEq, Eq)]
enum CompilationStage {
    Lex,
    Parse,
    Codegen,
}

fn parse_args() -> Option<CompilationStage> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file> [--lex | --parse | --codegen]", args[0]);
        return None;
    }

    let stage = args.iter().find_map(|arg| match arg.as_str() {
        "--lex" => Some(CompilationStage::Lex),
        "--parse" => Some(CompilationStage::Parse),
        "--codegen" => Some(CompilationStage::Codegen),
        _ => None,
    }).unwrap_or(CompilationStage::Parse);

    Some(stage)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("error: no input file provided");
        exit(1);
    }

    let filename = &args[1];
    let stage = parse_args().unwrap_or_else(|| exit(1));

    let source_code = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("error reading file '{}': {}", filename, err);
        exit(1);
    });

    let mut diagnostics = DiagnosticsManager::new(&source_code, filename);

    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.parse(&mut diagnostics);

    if !diagnostics.is_empty() {
        let _ = diagnostics.report();
        exit(1);
    }

    if stage == CompilationStage::Lex {
        println!("{:?}", tokens);
        exit(0);
    }

    let mut parser = Parser::new(tokens);
    let program = match parser.parse(&mut diagnostics) {
        Some(p) => p,
        None => {
            let _ = diagnostics.report();
            exit(1);
        }
    };

    if !diagnostics.is_empty() {
        let _ = diagnostics.report();
        exit(1);
    }

    if stage == CompilationStage::Parse {
        println!("{:#?}", program);
        exit(0);
    }

    exit(0);
}
