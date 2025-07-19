use common::*;
use language::*;
use zawarudo::zawarudo;

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

        let mut diagnostics = DiagnosticsManager::new(
            self.source_code, 
            self.filename
        );

        let mut tempgen = TempGen::new();

        let mut ctx = CompilerContext::new(
            &mut diagnostics,
            &mut tempgen
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
        let tac = match TacGeneratorStage::run(&mut program, &mut pipeline_ctx) {
            PipelineResult::Continue(tac) => tac,
            PipelineResult::Halt(result) => return result,
        };

        Ok(format!("{:#?}", tac))
    }


    pub fn preprocess(input_file: &std::path::Path) -> Result<std::path::PathBuf, String> {
        let preprocessed_file = input_file.with_extension("i");
        let status = std::process::Command::new("gcc")
            .args(["-E", "-P"])
            .arg(input_file)
            .arg("-o")
            .arg(&preprocessed_file)
            .status();

        match status {
            Err(e) => Err(format!("Failed to run preprocessor: {}", e)),
            Ok(s) if !s.success() => Err("Preprocessing failed".to_string()),
            Ok(_) => Ok(preprocessed_file),
        }
    }

    pub fn assemble_and_link(
        asm_file: &std::path::Path,
        input_file: &std::path::Path,
    ) -> Result<(), String> {
        let output_exe = input_file.with_extension("");
        let status = std::process::Command::new("gcc")
            .arg(asm_file)
            .arg("-o")
            .arg(&output_exe)
            .status();

        let _ = std::fs::remove_file(asm_file);

        match status {
            Err(e) => Err(format!("Failed to assemble/link: {}", e)),
            Ok(s) if !s.success() => Err("Assembly/linking failed".to_string()),
            Ok(_) => Ok(()),
        }
    }
}