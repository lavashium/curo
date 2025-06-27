use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::exit;

use common::*;
use frontend::*;
use backend::*;

#[derive(Debug, PartialEq, Eq)]
enum CompilationStage {
    Lex,
    Parse,
    Codegen,
}

struct Config {
    input_file: String,
    output_file: Option<String>,
    stage: CompilationStage,
}

fn parse_args() -> Option<Config> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file> [-o <output>] [--lex | --parse | --codegen]", args[0]);
        return None;
    }

    let mut input_file = None;
    let mut output_file = None;
    let mut stage = CompilationStage::Codegen;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: expected output file after '-o'");
                    return None;
                }
                output_file = Some(args[i].clone());
            }
            "--lex" => stage = CompilationStage::Lex,
            "--parse" => stage = CompilationStage::Parse,
            "--codegen" => stage = CompilationStage::Codegen,
            arg if !arg.starts_with('-') && input_file.is_none() => {
                input_file = Some(arg.to_string());
            }
            _ => {
                eprintln!("error: unrecognized or misplaced argument '{}'", args[i]);
                return None;
            }
        }
        i += 1;
    }

    let input_file = input_file?;
    Some(Config {
        input_file,
        output_file,
        stage,
    })
}

fn main() {
    let config = parse_args().unwrap_or_else(|| exit(1));

    let source_code = fs::read_to_string(&config.input_file).unwrap_or_else(|err| {
        eprintln!("error reading file '{}': {}", config.input_file, err);
        exit(1);
    });

    let mut diagnostics = DiagnosticsManager::new(&source_code, &config.input_file);

    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.parse(&mut diagnostics);

    if !diagnostics.is_empty() {
        let _ = diagnostics.report();
        exit(1);
    }

    if config.stage == CompilationStage::Lex {
        println!("{:#?}", tokens);
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

    if config.stage == CompilationStage::Parse {
        println!("{:#?}", program);
        exit(0);
    }

    let mut translator = Translator::new(program);
    let asm = translator.parse();
    let asm_str = asm.to_asm_string(1);

    if let Some(output_path) = config.output_file {
        let mut file = File::create(&output_path).unwrap_or_else(|err| {
            eprintln!("error: could not create output file '{}': {}", output_path, err);
            exit(1);
        });

        file.write_all(asm_str.as_bytes()).unwrap_or_else(|err| {
            eprintln!("error: failed to write to '{}': {}", output_path, err);
            exit(1);
        });
    } else {
        println!("{}", asm_str);
    }

    exit(0);
}