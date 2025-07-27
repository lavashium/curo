use super::*;
use accessors::accessors;
use language::*;

#[accessors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    severity: Severity,
    kind: DiagnosticKind,
    span: Span,
    children: Vec<Diagnostic>,
}

impl Diagnostic {
    pub fn new(severity: Severity, kind: DiagnosticKind, span: Span) -> Self {
        Diagnostic {
            severity,
            kind,
            span,
            children: Vec::new(),
        }
    }

    pub fn error(span: Span, kind: DiagnosticKind) -> Self {
        Self::new(Severity::Error, kind, span)
    }

    pub fn warning(span: Span, kind: DiagnosticKind) -> Self {
        Self::new(Severity::Warning, kind, span)
    }

    pub fn note(span: Span, kind: DiagnosticKind) -> Self {
        Self::new(Severity::Note, kind, span)
    }

    pub fn help(span: Span, kind: DiagnosticKind) -> Self {
        Self::new(Severity::Help, kind, span)
    }

    pub fn with(mut self, note: Diagnostic) -> Self {
        self.children.push(note);
        self
    }
}
