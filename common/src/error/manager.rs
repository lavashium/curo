use super::formats::*;
use super::*;
use accessors::accessors;
use constructors::constructors;
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticFormat {
    Normal,
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticConfiguration {
    formatter: DiagnosticFormat,
    source_code: String,
    filename: String,
}

#[accessors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticManager {
    diagnostics: Vec<Diagnostic>,
    config: DiagnosticConfiguration,
}

impl DiagnosticManager {
    pub fn new(config: DiagnosticConfiguration) -> Self {
        DiagnosticManager {
            diagnostics: Vec::new(),
            config
        }
    }

    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn report(&self) -> io::Result<()> {
        let mut stderr = io::stderr();

        let writer = &mut stderr;
        let config = self.config();
        let diags = self.diagnostics();

        match self.config.formatter() {
            DiagnosticFormat::Normal => NormalFormatter::write_diagnostics(writer, config, diags)
        }?;

        Ok(())
    }
}