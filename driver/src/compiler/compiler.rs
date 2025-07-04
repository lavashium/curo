use backend::*;
use common::DiagnosticsManager;
use frontend::*;
use middleend::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrCode {
    LexerError = 1,
    ParserError = 2,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PipelineStage {
    Lexer,
    Parser,
    TacGeneration,
    AssemblyGeneration,
    AssemblyAllocation,
    AssemblyLegalization,
    CodeEmission,
}

pub struct Compiler<'a> {
    source_code: &'a str,
    diagnostics: DiagnosticsManager,
}

impl<'a> Compiler<'a> {
    pub fn new(source_code: &'a str, filename: impl ToString) -> Self {
        Self {
            source_code: source_code,
            diagnostics: DiagnosticsManager::new(source_code, filename.to_string()),
        }
    }

    pub fn compile(&mut self, stage: PipelineStage) -> Result<String, ErrCode> {
        let mut lexer = Lexer::new(self.source_code);
        let tokens = lexer.parse(&mut self.diagnostics);

        if !self.diagnostics.is_empty() {
            let _ = self.diagnostics.report();
            return Err(ErrCode::LexerError);
        }

        if stage == PipelineStage::Lexer {
            return Ok(format!("{:#?}", tokens));
        }

        let mut parser = Parser::new(tokens);
        let program = parser.parse(&mut self.diagnostics);

        if !self.diagnostics.is_empty() {
            let _ = self.diagnostics.report();
            return Err(ErrCode::ParserError);
        }

        let program = program.expect("Parser returned None despite no diagnostics errors");

        if stage == PipelineStage::Parser {
            return Ok(format! {"{:#?}", program});
        }

        let mut generator = TacGenerator::new();
        let tac = generator.parse(program);

        if stage == PipelineStage::TacGeneration {
            return Ok(format! {"{:#?}", tac});
        }

        let translator = AsmGenerator::new();
        let asm_ast = translator.generate(tac);

        if stage == PipelineStage::AssemblyGeneration {
            return Ok(format! {"{:#?}", asm_ast});
        }

        let allocator = AsmAllocator::new();
        let (asm_allocated, stack_size) = allocator.allocate(asm_ast);

        if stage == PipelineStage::AssemblyAllocation {
            return Ok(format! {"{:#?}", asm_allocated});
        }

        let legalizer = AsmLegalizer::new(stack_size);
        let asm_legal = legalizer.legalize(asm_allocated);

        if stage == PipelineStage::AssemblyLegalization {
            return Ok(format! {"{:#?}", asm_legal});
        }

        let emitter = CodeEmitter::new();
        let asm = emitter.emit(asm_legal);

        Ok(asm)
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
