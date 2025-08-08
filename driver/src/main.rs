use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use driver::*;

fn parse_args() -> (PipelineStage, bool, bool, PathBuf) {
    let mut args = env::args().skip(1).peekable();
    let mut stage = PipelineStage::CodeEmission;
    let mut emit_asm = false;
    let mut compile_obj = false;

    while let Some(arg) = args.next_if(|a| a.starts_with('-')) {
        match arg.as_str() {
            "--lex" => stage = PipelineStage::Lexer,
            "--parse" => stage = PipelineStage::Parser,
            "--validate" => stage = PipelineStage::Analysis,
            "--tacky" => stage = PipelineStage::TacGeneration,
            "--codegen" => stage = PipelineStage::AssemblyGeneration,
            "-S" => emit_asm = true,
            "-c" => compile_obj = true,
            _ => {
                eprintln!("Unknown option: {arg}");
                exit(1);
            }
        }
    }

    let input_file = match args.next() {
        Some(f) => PathBuf::from(f),
        None => {
            eprintln!("Missing input file");
            exit(1);
        }
    };

    (stage, emit_asm, compile_obj, input_file)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (stage, emit_asm, compile_obj, input_file) = parse_args();
    
    let preprocessed = CompilerDriver::preprocess(&input_file)
        .unwrap_or_else(|e| { eprintln!("{e}"); exit(1) });
    
    let source = fs::read_to_string(&preprocessed)
        .unwrap_or_else(|e| { eprintln!("Failed to read {preprocessed:?}: {e}"); exit(1) });
    
    let filename = &input_file.to_string_lossy();
    let has_main = source.contains("main");

    let mut driver = CompilerDriver::new(&source, filename);
    let result = driver.compile(stage.clone());
    
    fs::remove_file(preprocessed).ok();
    
    match result {
        Err(code) => exit(code as i32),
        Ok(output) if stage != PipelineStage::CodeEmission => {
            println!("{output}");
            Ok(())
        }
        Ok(asm) => {
            let asm_file = input_file.with_extension("s");
            fs::write(&asm_file, asm)?;

            if emit_asm {
                return Ok(());
            }

            if compile_obj {
                CompilerDriver::assemble(&asm_file)?;
            } else {
                if has_main {
                    let exe_name = input_file.with_extension("").to_string_lossy().into_owned();
                    CompilerDriver::link(&asm_file, &exe_name)?;
                } else {
                    CompilerDriver::assemble(&asm_file)?;
                }
            }

            fs::remove_file(asm_file)?;
            Ok(())
        }
    }
}