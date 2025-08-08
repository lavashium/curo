use language::*;

use super::*;

pub struct NormalFormatter;

impl DiagnosticFormatter for NormalFormatter {
    fn format_severity(severity: &Severity) -> String {
        match severity {
            Severity::Error   => "error",
            Severity::Help    => "help",
            Severity::Note    => "note",
            Severity::Warning => "warning",
        }.into()
    }

    fn format_lexical(error: &LexicalError) -> String {
        match error {
            LexicalError::UnknownToken { token } => {
                format!(
                    "unknown token `{}`", 
                    token.lexeme()
                )
            }
            LexicalError::UnexpectedEof => {
                format!("unexpected end of file")
            }
            LexicalError::UnterminatedMultilineComment => {
                format!("unterminated multi-line comment")
            }
            LexicalError::UnterminatedStringLiteral => {
                format!("unterminated string literal")
            }
            LexicalError::UnterminatedCharLiteral => {
                format!("unterminated character literal")
            }
            LexicalError::InvalidNumberLiteral { literal } => {
                format!(
                    "invalid number literal `{}`", 
                    literal
                )
            }
            LexicalError::InvalidCharLiteral { literal } => {
                format!(
                    "invalid character literal `{}`", 
                    literal
                )
            }
            LexicalError::NonAsciiCharacter => {
                format!("non-ASCII character")
            }
        }
    }

    fn format_syntax(error: &SyntaxError) -> String {
        match error {
            SyntaxError::UnexpectedToken { found, expected } => {
                format!(
                    "unexpected token `{}`, expected {}",
                    found.lexeme(),
                    into_csl(lower_tokenkind, expected)
                )
            },
            SyntaxError::ExpectedToken { expected, found } => {
                format!(
                    "expected {}, found `{}`",
                    lower_tokenkind(expected),
                    found.lexeme()
                )
            },
            SyntaxError::UnexpectedDeclaration => {
                format!("unexpected declaration")
            },
            SyntaxError::UnexpectedStatement => {
                format!("unexpected statement")
            },
            SyntaxError::InvalidLValue => {
                format!("invalid left-hand side value")
            },
        }
    }
    
    fn format_semantic(error: &SemanticError) -> String {
        match error {
            SemanticError::DuplicateDeclaration { name } => {
                format!(
                    "duplicate declaration of `{}`", 
                    name
                )
            },
            SemanticError::UseOfUndeclared { name } => {
                format!(
                    "use of undeclared identifier `{}`", 
                    name
                )
            },
            SemanticError::ConflictingDeclarations { name } => {
                format!(
                    "conflicting declarations for `{}`", 
                    name
                )
            },
            SemanticError::TypeMismatch { expected, found } => {
                format!(
                    "type mismatch: expected `{}` found `{}`",
                    classify_type(expected),
                    classify_type(found),
                )
            },
            SemanticError::InvalidType { ty } => {
                format!(
                    "invalid type `{}`", 
                    classify_type(ty)
                )
            },
            SemanticError::IncompleteType { ty } => {
                format!(
                    "incomplete type `{}`", 
                    classify_type(ty)
                )
            },
            SemanticError::InvalidAssignment => {
                format!("invalid assignment")
            },
            SemanticError::InvalidCast { from, to } => {
                format!(
                    "invalid cast from `{}` to `{}`",
                    classify_type(from),
                    classify_type(to),
                )
            },
            SemanticError::InvalidFunctionCall { name, expected_args, found_args } => {
                format!(
                    "invalid function call `{}`: expected {} argument(s), found {}",
                    name,
                    expected_args, 
                    found_args
                )
            },
            SemanticError::NonConstInitializer => {
                format!("initializer is not a constant expression")
            },
            SemanticError::ReturnTypeMismatch { expected, found } => {
                format!(
                    "return type mismatch: expected `{}`, found `{}`", 
                    classify_type(expected), 
                    classify_type(found)
                )
            },
            SemanticError::VoidValueUsed => {
                format!("void value not ignored as it ought to be")
            }
            SemanticError::InvalidArraySubscript => {
                format!("invalid array subscript")
            }
            SemanticError::IncompatiblePointerTypes => {
                format!("incompatible pointer types")
            }
            SemanticError::RedeclaredSymbol { name } => {
                format!(
                    "redeclared symbol `{}`", 
                    name
                )
            }
            SemanticError::UndefinedFunction { name } => {
                format!(
                    "use of undeclared function `{}`", 
                    name
                )
            }
            SemanticError::FunctionRedefinition { name } => {
                format!(
                    "redefinition of function `{}`", 
                    name
                )
            }
            SemanticError::NestedFunctionDefinition => {
                format!("nested function definitions are not allowed")
            }
            SemanticError::InvalidStorageSpecifier => {
                format!("invalid storage class specifier")
            },
            SemanticError::ConflictingStorageSpecifiers => {
                format!("multiple storage classes in declaration")
            },
            SemanticError::InvalidTypeSpecifierCount { expected, found } => {
                format!(
                    "invalid type specifier: expected {} type specifier(s), found {}",
                    expected, found
                )
            },
            SemanticError::MissingTypeSpecifier => {
                format!("missing type specifier in declaration")
            },
            SemanticError::InvalidStorageSpecifierLocation => {
                format!("storage class specifier not allowed in this context")
            },
            SemanticError::ConflictingTypeSpecifiers => {
                format!("conflicting type specifiers")
            },
        }
    }
    
    fn format_control_flow(error: &ControlFlowError) -> String {
        match error {
            ControlFlowError::BreakOutsideLoop => {
                format!("`break` outside of loop")
            }
            ControlFlowError::ContinueOutsideLoop => {
                format!("`continue` outside of loop")
            }
            ControlFlowError::ReturnOutsideFunction => {
                format!("`return` outside of function")
            }
            ControlFlowError::CaseOutsideSwitch => {
                format!("`case` label outside of switch")
            }
            ControlFlowError::DuplicateCaseLabel => {
                format!("duplicate case label")
            }
            ControlFlowError::DefaultLabelAlreadyUsed => {
                format!("`default` label already used")
            }
            ControlFlowError::GotoUndefinedLabel { label } => {
                format!(
                    "goto to undefined label `{}`", 
                    label
                )
            }
        }
    }
    
    fn format_preprocessor(error: &PreprocessorError) -> String {
        match error {
            PreprocessorError::Message(message) => {
                message.clone()
            },
        }
    }
    
    fn format_warning(error: &Warning) -> String {
        match error {
            Warning::UnusedVariable { name } => {
                format!(
                    "unused variable `{}`", 
                    name
                )
            }
            Warning::UnusedFunction { name } => {
                format!(
                    "unused function `{}`", 
                    name
                )
            }
            Warning::VariableShadowing { name } => {
                format!(
                    "variable `{}` shadows a previous declaration", 
                    name
                )
            }
            Warning::ImplicitConversion { from, to } => {
                format!(
                    "implicit conversion from `{}` to `{}`", 
                    from, 
                    to
                )
            }
            Warning::SignedUnsignedComparison => {
                format!("comparison between signed and unsigned integer expressions")
            }
            Warning::Message(message) => {
                message.clone()
            },
        }
    }
    
    fn format_internal(error: &InternalError) -> String {
        match error {
            InternalError::Message(message) => {
                message.clone()
            },
        }
    }
    
    fn format_custom(error: &CustomError) -> String {
        match error {
            CustomError::Message(message) => {
                message.clone()
            },
        }
    }
}

impl DiagnosticWriter for NormalFormatter {}

#[allow(dead_code)]
fn classify_tokenkind(token: &TokenKind) -> &str {
    match token {
        TokenKind::Keyword(_)      => "keyword",
        TokenKind::Identifier(_)   => "identifier",
        TokenKind::Operator(_)     => "operator",
        TokenKind::Punctuation(_)  => "punctuation",
        TokenKind::Constant(_)     => "constant",
        TokenKind::Unknown(_)      => "unknown",
        TokenKind::EOF             => "<EOF>",
        TokenKind::Irrelevant      => unreachable!("Irrelevant tokens should not reach formatter"),
    }
}

#[allow(dead_code)]
fn lower_tokenkind(token: &TokenKind) -> &str {
    match token {
        TokenKind::Identifier(id)        => id,
        TokenKind::Constant(c)           => c,
        TokenKind::Unknown(u)            => u,
        TokenKind::EOF                   => "<EOF>",

        TokenKind::Keyword(kw) => match kw {
            KeywordKind::Int             => "int",
            KeywordKind::Return          => "return",
            KeywordKind::Void            => "void",
            KeywordKind::If              => "if",
            KeywordKind::Else            => "else",
            KeywordKind::Do              => "do",
            KeywordKind::While           => "while",
            KeywordKind::For             => "for",
            KeywordKind::Break           => "break",
            KeywordKind::Continue        => "continue",
            KeywordKind::Static          => "static",
            KeywordKind::Extern          => "extern",
        },

        TokenKind::Operator(op) => match op {
            OperatorKind::Tilde          => "~",
            OperatorKind::Minus          => "-",
            OperatorKind::Plus           => "+",
            OperatorKind::Asterisk       => "*",
            OperatorKind::Slash          => "/",
            OperatorKind::Percent        => "%",
            OperatorKind::Exclamation    => "!",
            OperatorKind::LessThan       => "<",
            OperatorKind::GreaterThan    => ">",
            OperatorKind::Equal          => "=",
            OperatorKind::LogicalAnd     => "&&",
            OperatorKind::LogicalOr      => "||",
            OperatorKind::EqualEqual     => "==",
            OperatorKind::NotEqual       => "!=",
            OperatorKind::LessEqual      => "<=",
            OperatorKind::GreaterEqual   => ">=",
            OperatorKind::Question       => "?",
        },

        TokenKind::Punctuation(punct) => match punct {
            PunctuationKind::Semicolon   => ";",
            PunctuationKind::OpenParen   => "(",
            PunctuationKind::CloseParen  => ")",
            PunctuationKind::OpenBrace   => "{",
            PunctuationKind::CloseBrace  => "}",
            PunctuationKind::Colon       => ":",
            PunctuationKind::Comma       => ",",
        },

        TokenKind::Irrelevant => unreachable!("Irrelevant tokens should not reach formatter"),
    }
}

#[allow(dead_code)]
fn classify_type(ty: &AstType) -> &str {
    match ty {
        AstType::Int                     => "int",
        AstType::FunType(_)              => "function",
    }
}