use crate::error::manager::DiagnosticsManager;
use crate::parser::Parser;
use crate::token::*;
use crate::ast::*;
use crate::*;

macro_rules! error_expect {
    ($parser:expr, $kind:expr, $diagnostics:expr) => {{
        match $parser.source_tokens.consume() {
            Some(token) if token.kind() == &$kind => Some(token),
            Some(found) => {
                $diagnostics.push(errkind_error!(
                    found.span,
                    error_expected_token!($kind.clone(), found.clone())
                ));
                None
            }
            None => {
                $diagnostics.push(errkind_error!(
                    Span::default(),
                    error_unexpected_eof!()
                ));
                None
            }
        }
    }};
}

macro_rules! error_consume_unwrap {
    ($parser:expr, $kind:ident, $diagnostics:expr) => {{
        match $parser.source_tokens.consume() {
            Some(token) => match token.kind() {
                TokenKind::$kind(inner) => Some(inner.to_owned()),
                _ => {
                    $diagnostics.push(errkind_error!(
                        token.span,
                        error_unexpected_token!(token.clone(), [TokenKind::$kind("".to_string())])
                    ));
                    None
                }
            },
            None => {
                $diagnostics.push(errkind_error!(
                    Span::default(),
                    error_unexpected_eof!()
                ));
                None
            }
        }
    }};
}

pub fn parse_program(parser: &mut Parser, diagnostics: &mut DiagnosticsManager) -> Option<Program> {
    let function = parse_function(parser, diagnostics)?;

    if parser.source_tokens.any_tokens() {
        for token in parser.source_tokens.iter() {
            diagnostics.push(errkind_error!(
                token.span,
                error_custom!(
                    "Leftover tokens"
                )
            ));
        }
        return None;
    }

    Some(Program {
        function_definition: function
    })
}

fn parse_function(parser: &mut Parser, diagnostics: &mut DiagnosticsManager) -> Option<Function> {
    error_expect!(parser, token_keyword!(Int), diagnostics)?;

    let identifier = parse_identifier(parser, diagnostics)?;

    error_expect!(parser, token_punctuation!(OpenParen), diagnostics)?;
    error_expect!(parser, token_keyword!(Void), diagnostics)?;
    error_expect!(parser, token_punctuation!(CloseParen), diagnostics)?;
    error_expect!(parser, token_punctuation!(OpenBrace), diagnostics)?;

    let statement = parse_statement(parser, diagnostics)?;

    error_expect!(parser, token_punctuation!(CloseBrace), diagnostics)?;

    Some(Function {
        identifier_name: identifier,
        statement_body: statement,
    })
}

fn parse_statement(parser: &mut Parser, diagnostics: &mut DiagnosticsManager) -> Option<Statement> {
    match parser.source_tokens.peek()?.kind() {

        TokenKind::Keyword(KeywordKind::Return) => {
            error_expect!(parser, token_keyword!(Return), diagnostics)?;
            let expression = parse_expression(parser, diagnostics)?;
            error_expect!(parser, token_punctuation!(Semicolon), diagnostics)?;
            Some(Statement::Return {
                expression: expression 
            })
        },

        _ => None,
    }
}

fn parse_expression(parser: &mut Parser, diagnostics: &mut DiagnosticsManager) -> Option<Expression> {
    match parser.source_tokens.peek()?.kind() {

        TokenKind::Constant(_) => {
            let constant = parse_constant(parser, diagnostics)?;
            Some(Expression::Constant {
                constant: constant
            })
        }

        _ => None
    }
}

fn parse_identifier(parser: &mut Parser, diagnostics: &mut DiagnosticsManager) -> Option<String> {
    error_consume_unwrap!(parser, Identifier, diagnostics)
}

fn parse_constant(parser: &mut Parser, diagnostics: &mut DiagnosticsManager) -> Option<String> {
    error_consume_unwrap!(parser, Constant, diagnostics)
}