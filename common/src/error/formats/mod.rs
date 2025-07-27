mod fmt_normal;

pub use fmt_normal::*;

use super::*;

pub trait DiagnosticFormatter {
    fn format_diagnostics(error: &DiagnosticKind) -> String {
        match error {
            DiagnosticKind::Lexical(error)      => Self::format_lexical(error),
            DiagnosticKind::Syntax(error)       => Self::format_syntax(error),
            DiagnosticKind::Semantic(error)     => Self::format_semantic(error),
            DiagnosticKind::ControlFlow(error)  => Self::format_control_flow(error),
            DiagnosticKind::Preprocessor(error) => Self::format_preprocessor(error),
            DiagnosticKind::Warning(error)      => Self::format_warning(error),
            DiagnosticKind::Internal(error)     => Self::format_internal(error),
            DiagnosticKind::Custom(error)       => Self::format_custom(error),
        }
    }

    fn format_severity(severity: &Severity) -> String;
    fn format_lexical(error: &LexicalError) -> String;
    fn format_syntax(error: &SyntaxError) -> String;
    fn format_semantic(error: &SemanticError) -> String;
    fn format_control_flow(error: &ControlFlowError) -> String;
    fn format_preprocessor(error: &PreprocessorError) -> String;
    fn format_warning(error: &Warning) -> String;
    fn format_internal(error: &InternalError) -> String;
    fn format_custom(error: &CustomError) -> String;
}

pub trait DiagnosticWriter: DiagnosticFormatter {
    fn write_diagnostics<W: std::io::Write>(writer: &mut W, config: &DiagnosticConfiguration, diags: &Vec<Diagnostic>) -> std::io::Result<()> {
        for diag in diags {
            Self::write_diagnostic(writer, config, diag)?;
        }
        Ok(())
    }

    fn write_diagnostic<W: std::io::Write>(writer: &mut W, config: &DiagnosticConfiguration, diag: &Diagnostic) -> std::io::Result<()> {
        let lines: Vec<&str> = config.source_code().lines().collect();
        let line_index = diag.span().get_start_line();
        let col_index = diag.span().get_start_col();
        let line_display = line_index + 1;
        let col_display = col_index + 1;

        let severity_str = Self::format_severity(diag.severity());
        let formatted_message = Self::format_diagnostics(diag.kind());

        writeln!(
            writer,
            "{}:{}:{}: {}: {}",
            config.filename(), 
            line_display, 
            col_display, 
            severity_str, 
            formatted_message
        )?;

        if line_index < lines.len() {
            let source_line = lines[line_index];
            writeln!(writer, "{:5} | {}", line_display, source_line)?;

            let mut underline = String::new();
            let line_len = source_line.len();
            
            for _ in 0..col_index {
                underline.push(' ');
            }

            let underline_len = if diag.span().get_start_line() == diag.span().get_end_line() {
                let end_col = diag.span().get_end_col();
                (end_col - col_index).min(line_len - col_index)
            } else {
                line_len - col_index
            };

            if underline_len > 0 {
                underline.push('^');
                for _ in 1..underline_len {
                    underline.push('~');
                }
            } else {
                underline.push('^');
            }

            writeln!(writer, "      | {}", underline)?;
        }

        Ok(())
    }
}

pub fn into_csl<T, F: Fn(&T) -> &str>(f: F, input: &Vec<T>) -> String {
    input
        .iter()
        .map(|item| f(item))
        .collect::<Vec<_>>()
        .join(", ")
}