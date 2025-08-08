use common::*;
use language::*;
use zawarudo::zawarudo;

use std::path::PathBuf;

use crate::*;
use super::stages::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrCode {
    Succes = 0,
    LexerError = 1,
    ParserError = 2,
    SemanticError = 3,
}

pub struct CompilerDriver<'a> {
    source_code: &'a str,
    filename: &'a str
}

impl<'a> CompilerDriver<'a> {
    pub fn new(source_code: &'a str, filename: &'a str) -> Self {
        Self {
            source_code,
            filename
        }
    }

    #[zawarudo(label = "main")]
    pub fn compile(&mut self, stage: PipelineStage) -> Result<String, ErrCode> {

        let diagnostic_configuration = DiagnosticConfiguration::new(
            DiagnosticFormat::Normal,
            self.source_code.into(),
            self.filename.into(),
        );

        let mut diagnostics = DiagnosticManager::new(
            diagnostic_configuration
        );

        let mut tempgen = TempGen::new();

        let mut ctx = CompilerContext::new(
            &mut diagnostics,
            &mut tempgen,
            SymbolTable::new(),
        );

        let mut pipeline_ctx = PipelineContext::new(
            &mut ctx, 
            stage, 
            String::new(), 
            ErrCode::Succes
        );

        let mut source = self.source_code;
        let tokens = match LexerStage::run(&mut source, &mut pipeline_ctx) {
            PipelineResult::Continue(tokens) => tokens,
            PipelineResult::Halt(result) => return result,
        };

        let mut tokens = tokens;
        let program = match ParserStage::run(&mut tokens, &mut pipeline_ctx) {
            PipelineResult::Continue(program) => program,
            PipelineResult::Halt(result) => return result,
        };

        let mut program = program;
        let program = match AnalysisStage::run(&mut program, &mut pipeline_ctx) {
            PipelineResult::Continue(program) => program,
            PipelineResult::Halt(result) => return result,
        };

        let mut program = program;
        let tac = match TacGeneratorStage::run(&mut program, &mut pipeline_ctx) {
            PipelineResult::Continue(tac) => tac,
            PipelineResult::Halt(result) => return result,
        };

        let mut tac = tac;
        let asm = match X86_64::run(&mut tac, &mut pipeline_ctx) {
            PipelineResult::Continue(asm) => asm,
            PipelineResult::Halt(result) => return result,
        };

        Ok(asm)
    }

    pub fn preprocess(input_file: &std::path::Path) -> Result<std::path::PathBuf, String> {
        let output = input_file.with_extension("i");
        std::process::Command::new("gcc")
            .args(["-E", "-P", "-o", &output.to_string_lossy(), &input_file.to_string_lossy()])
            .status()
            .map_err(|e| format!("Preprocessor failed: {e}"))
            .and_then(|s| if s.success() { Ok(output) } else { Err("Preprocessor returned non-zero".into()) })
    }

    pub fn assemble(asm_file: &std::path::Path) -> Result<PathBuf, String> {
        let obj_file = asm_file.with_extension("o");
        run_command("gcc", &["-c", &asm_file.to_string_lossy(), "-o", &obj_file.to_string_lossy()])
            .map(|_| obj_file)
    }

    pub fn link(asm_file: &std::path::Path, exe_name: &str) -> Result<(), String> {
        run_command("gcc", &[&asm_file.to_string_lossy(), "-o", exe_name])
    }
}

fn run_command(cmd: &str, args: &[&str]) -> Result<(), String> {
    std::process::Command::new(cmd)
        .args(args)
        .status()
        .map_err(|e| format!("{cmd} failed: {e}"))
        .and_then(|s| if s.success() { Ok(()) } else { Err(format!("{cmd} returned non-zero")) })
}