use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use driver::*;
fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("Usage: compiler_driver [--lex|--parse|--codegen|-S|-c] <file.c>");
        exit(1);
    }

    let mut stage = PipelineStage::CodeEmission;
    let mut emit_assembly_only = false;
    let mut compile_to_object = false;

    while !args.is_empty() && args[0].starts_with('-') {
        let arg = args.remove(0);
        match arg.as_str() {
            "--lex" => stage = PipelineStage::Lexer,
            "--parse" => stage = PipelineStage::Parser,
            "--validate" => stage = PipelineStage::Analysis,
            "--tacky" => stage = PipelineStage::TacGeneration,
            "--codegen" => stage = PipelineStage::AssemblyGeneration,
            "--allocation" => stage = PipelineStage::AssemblyAllocation,
            "--legalization" => stage = PipelineStage::AssemblyLegalization,
            "-S" => {
                stage = PipelineStage::CodeEmission;
                emit_assembly_only = true;
            }
            "-c" => {
                stage = PipelineStage::CodeEmission;
                compile_to_object = true;
            }
            _ => {
                eprintln!("Unknown option: {}", arg);
                exit(1);
            }
        }
    }

    if args.len() != 1 {
        eprintln!("Expected single input file. Got: {:?}", args);
        exit(1);
    }

    let input_file = PathBuf::from(&args[0]);
    if !input_file.exists() {
        eprintln!("Input file does not exist: {}", input_file.display());
        exit(1);
    }

    let preprocessed_file = match Compiler::preprocess(&input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    let source_code = match fs::read_to_string(&preprocessed_file) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Failed to read preprocessed file: {}", e);
            let _ = fs::remove_file(&preprocessed_file);
            exit(1);
        }
    };

    let filename = input_file
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let mut compiler = Compiler::new(&source_code, filename);

    let result = compiler.compile(stage.clone());

    let _ = fs::remove_file(&preprocessed_file);

    match result {
        Err(code) => {
            exit(code as i32);
        }
        Ok(output) => {
            if !matches!(stage, PipelineStage::CodeEmission) {
                println!("{}", output);
                exit(0);
            }

            let asm_file = input_file.with_extension("s");

            if let Err(e) = fs::write(&asm_file, output) {
                eprintln!("Failed to write assembly file: {}", e);
                exit(1);
            }

            if emit_assembly_only {
                exit(0);
            }

            if compile_to_object {
                let object_file = input_file.with_extension("o");
                let status = std::process::Command::new("gcc")
                    .arg("-c")
                    .arg(&asm_file)
                    .arg("-o")
                    .arg(&object_file)
                    .status();

                let _ = std::fs::remove_file(&asm_file);

                match status {
                    Err(e) => {
                        eprintln!("Failed to compile to object file: {}", e);
                        exit(1);
                    }
                    Ok(s) if !s.success() => {
                        eprintln!("Compilation to object file failed");
                        exit(1);
                    }
                    Ok(_) => exit(0),
                }
            }

            if let Err(e) = Compiler::assemble_and_link(&asm_file, &input_file) {
                eprintln!("{}", e);
                exit(1);
            }

            exit(0);
        }
    }
}

