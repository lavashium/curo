use super::convert::*;
use language::*;

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

    UnexpectedEof,
    InvalidType(String),
    InvalidGenericType(GenericKind),
    Custom(String),
}

impl DiagnosticKind {
    pub fn message(&self) -> String {
        match self {
            DiagnosticKind::UnknownToken(token) => {
                format!("unknown {} '{}'", token.kind.to_user_string(), token.lexeme)
            }

            DiagnosticKind::UnexpectedToken { found, expected } => {
                let expected_str = expected
                    .iter()
                    .map(|k| k.to_user_string())
                    .collect::<Vec<_>>()
                    .join(" or ");
                format!("expected {}, found '{}'", expected_str, found.lexeme)
            }

            DiagnosticKind::ExpectedToken { expected, found } => format!(
                "expected '{}', found '{}'",
                expected.to_user_string(),
                found.lexeme
            ),

            DiagnosticKind::UnexpectedGeneric { found, expected } => {
                let expected_str = expected
                    .iter()
                    .map(|k| k.to_user_string())
                    .collect::<Vec<_>>()
                    .join(" or ");
                format!("expected {}, found '{}'", expected_str, found.lexeme)
            }

            DiagnosticKind::ExpectedGeneric { expected, found } => format!(
                "expected {}, found '{}'",
                expected.to_user_string(),
                found.lexeme
            ),

            DiagnosticKind::UnexpectedEof => "unexpected end of file".to_string(),

            DiagnosticKind::InvalidType(ty) => format!("invalid type '{}'", ty),
            DiagnosticKind::InvalidGenericType(ty) => {
                format!("invalid type {}", ty.to_user_string())
            }

            DiagnosticKind::Custom(msg) => msg.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub span: Span,
    pub severity: Severity,
    pub kind: DiagnosticKind,
    pub children: Vec<Diagnostic>,
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

#[macro_export]
macro_rules! error_unknown_token {
    ($token:expr) => {
        $crate::error::diagnostic::DiagnosticKind::UnknownToken($token)
    };
}

#[macro_export]
macro_rules! error_unexpected_token {
    ($found:expr, [$($expected:expr),+ $(,)?]) => {
        $crate::error::diagnostic::DiagnosticKind::UnexpectedToken {
            found: $found,
            expected: vec![$($expected),+],
        }
    };
}

#[macro_export]
macro_rules! error_expected_token {
    ($expected:expr, $found:expr) => {
        $crate::error::diagnostic::DiagnosticKind::ExpectedToken {
            expected: $expected,
            found: $found,
        }
    };
}

#[macro_export]
macro_rules! error_unexpected_eof {
    () => {
        $crate::error::diagnostic::DiagnosticKind::UnexpectedEof
    };
}

#[macro_export]
macro_rules! error_invalid_type {
    ($ty:expr) => {
        $crate::error::diagnostic::DiagnosticKind::InvalidType($ty.to_string())
    };
}

#[macro_export]
macro_rules! error_custom {
    ($msg:expr) => {
        $crate::error::diagnostic::DiagnosticKind::Custom($msg.to_string())
    };
}

#[macro_export]
macro_rules! errkind_error {
    ($span:expr, $kind:expr) => {
        $crate::error::diagnostic::Diagnostic::error($span, $kind)
    };
}

#[macro_export]
macro_rules! errkind_warning {
    ($span:expr, $kind:expr) => {
        $crate::error::diagnostic::Diagnostic::warning($span, $kind)
    };
}

#[macro_export]
macro_rules! errkind_note {
    ($span:expr, $msg:expr) => {
        $crate::error::diagnostic::Diagnostic::note($span, $msg)
    };
}

#[macro_export]
macro_rules! errkind_help {
    ($span:expr, $msg:expr) => {
        $crate::error::diagnostic::Diagnostic::help($span, $msg)
    };
}

#[macro_export]
macro_rules! error_expected_generic {
    ($expected:expr, $found:expr) => {
        $crate::error::diagnostic::DiagnosticKind::ExpectedGeneric {
            expected: $expected,
            found: $found,
        }
    };
}

#[macro_export]
macro_rules! error_unexpected_generic {
    ($found:expr, [$($expected:expr),+ $(,)?]) => {
        $crate::error::diagnostic::DiagnosticKind::UnexpectedGeneric {
            found: $found,
            expected: vec![$($expected),+],
        }
    };
}

#[macro_export]
macro_rules! error_invalid_generic_type {
    ($ty:expr) => {
        $crate::error::diagnostic::DiagnosticKind::InvalidGenericType($ty)
    };
}
