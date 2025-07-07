use super::convert::*;
use accessors::accessors;
use constructors::constructors;
use language::*;

#[constructors]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Note => write!(f, "note"),
            Severity::Help => write!(f, "help"),
        }
    }
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticKind {
    UnknownToken(Token),
    UnexpectedToken {
        found: Token,
        expected: Vec<TokenKind>,
    },
    ExpectedToken {
        expected: TokenKind,
        found: Token,
    },

    UnexpectedGeneric {
        found: Token,
        expected: Vec<GenericKind>,
    },
    ExpectedGeneric {
        expected: GenericKind,
        found: Token,
    },

    DuplicateVariableDeclaration {
        name: String,
    },
    UseOfUndeclaredVariable {
        name: String,
    },

    UnexpectedEof,
    InvalidType(String),
    InvalidGenericType(GenericKind),
    Custom(String),
}

impl DiagnosticKind {
    pub fn message(&self) -> String {
        match self {
            DiagnosticKind::UnknownToken(token) => {
                format!(
                    "unknown {} '{}'",
                    token.kind().to_user_string(),
                    token.lexeme()
                )
            }

            DiagnosticKind::UnexpectedToken { found, expected } => {
                let expected_str = expected
                    .iter()
                    .map(|k| k.to_user_string())
                    .collect::<Vec<_>>()
                    .join(" or ");
                format!("expected {}, found '{}'", expected_str, found.lexeme())
            }

            DiagnosticKind::ExpectedToken { expected, found } => format!(
                "expected '{}', found '{}'",
                expected.to_user_string(),
                found.lexeme()
            ),

            DiagnosticKind::UnexpectedGeneric { found, expected } => {
                let expected_str = expected
                    .iter()
                    .map(|k| k.to_user_string())
                    .collect::<Vec<_>>()
                    .join(" or ");
                format!("expected {}, found '{}'", expected_str, found.lexeme())
            }

            DiagnosticKind::ExpectedGeneric { expected, found } => format!(
                "expected {}, found '{}'",
                expected.to_user_string(),
                found.lexeme()
            ),
            
            DiagnosticKind::DuplicateVariableDeclaration { name } => {
                format!("duplicate variable declaration '{}'", name)
            }
            DiagnosticKind::UseOfUndeclaredVariable { name } => {
                format!("use of undeclared variable '{}'", name)
            }

            DiagnosticKind::UnexpectedEof => "unexpected end of file".to_string(),

            DiagnosticKind::InvalidType(ty) => format!("invalid type '{}'", ty),
            DiagnosticKind::InvalidGenericType(ty) => {
                format!("invalid type {}", ty.to_user_string())
            }

            DiagnosticKind::Custom(msg) => msg.clone(),
        }
    }
}

#[accessors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    span: Span,
    severity: Severity,
    kind: DiagnosticKind,
    children: Vec<Diagnostic>,
}

impl Diagnostic {
    pub fn new(span: Span, severity: Severity, kind: DiagnosticKind) -> Self {
        Diagnostic {
            span,
            severity,
            kind,
            children: Vec::new(),
        }
    }

    pub fn error(span: Span, kind: DiagnosticKind) -> Self {
        Self::new(span, Severity::Error, kind)
    }

    pub fn warning(span: Span, kind: DiagnosticKind) -> Self {
        Self::new(span, Severity::Warning, kind)
    }

    pub fn note(span: Span, message: impl Into<String>) -> Self {
        Self::new(span, Severity::Note, DiagnosticKind::Custom(message.into()))
    }

    pub fn help(span: Span, message: impl Into<String>) -> Self {
        Self::new(span, Severity::Help, DiagnosticKind::Custom(message.into()))
    }

    pub fn message(&self) -> String {
        self.kind.message()
    }

    pub fn with(mut self, note: Diagnostic) -> Self {
        self.children.push(note);
        self
    }
}
