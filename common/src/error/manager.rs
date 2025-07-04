use super::diagnostic::Diagnostic;
use std::io::{self, Write};
use accessors::accessors;

#[accessors]
#[derive(Debug, Default)]
pub struct DiagnosticsManager {
    diagnostics: Vec<Diagnostic>,
    source_code: String,
    filename: String,
}

impl DiagnosticsManager {
    pub fn new(source_code: impl ToString, filename: impl ToString) -> Self {
        let source_code = source_code.to_string();
        let filename = filename.to_string();
        DiagnosticsManager {
            diagnostics: Vec::new(),
            source_code,
            filename,
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
        let mut writer = DiagnosticWriter::new(&self.source_code, &self.filename, &mut stderr);

        for diag in &self.diagnostics {
            writer.write_diagnostic(diag)?;
            for child in diag.children() {
                writer.write_diagnostic(child)?;
            }
        }
        Ok(())
    }
}

#[accessors]
struct DiagnosticWriter<'a, W: Write> {
    filename: &'a str,
    writer: &'a mut W,
    lines: Vec<&'a str>,
}

impl<'a, W: Write> DiagnosticWriter<'a, W> {
    fn new(source: &'a str, filename: &'a str, writer: &'a mut W) -> Self {
        let lines = source.lines().collect();
        DiagnosticWriter {
            filename,
            writer,
            lines,
        }
    }

    fn write_diagnostic(&mut self, diag: &Diagnostic) -> io::Result<()> {
        let line = diag.span().start_line() + 1;
        let col = diag.span().start_col() + 1;
        writeln!(
            self.writer,
            "{}:{}:{}: {}: {}",
            self.filename,
            line,
            col,
            diag.severity(),
            diag.message()
        )?;

        if diag.span().start_line() < &self.lines.len() {
            let source_line = self.lines[diag.span().get_start_line()];
            writeln!(self.writer, "{:5} | {}", line, source_line)?;

            let underline = self.create_underline(diag);
            writeln!(self.writer, "      | {}", underline)?;
        }
        Ok(())
    }

    fn create_underline(&self, diag: &Diagnostic) -> String {
        let mut underline = String::new();
        let line_len = self.lines[diag.span().get_start_line()].len();

        for _ in 0..diag.span().get_start_col() {
            underline.push(' ');
        }

        let underline_len = if diag.span().start_line() == diag.span().end_line() {
            (diag.span().end_col() - diag.span().start_col()).min(line_len - diag.span().start_col())
        } else {
            line_len - diag.span().start_col()
        };

        if underline_len > 0 {
            underline.push('^');
            for _ in 1..underline_len {
                underline.push('~');
            }
        } else {
            underline.push('^');
        }

        underline
    }
}
