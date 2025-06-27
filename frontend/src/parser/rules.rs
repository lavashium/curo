use crate::parser::Parser;
use common::error::manager::DiagnosticsManager;
use common::*;
use language::ast::*;
use language::token::*;
use language::*;

macro_rules! error_expect {
    ($self:expr, $kind:expr) => {{
        match $self.parser.source_tokens.consume() {
            Some(token) if token.kind() == &$kind => Some(token),
            Some(found) => {
                $self.diagnostics.push(errkind_error!(
                    found.span,
                    error_expected_token!($kind.clone(), found.clone())
                ));
                None
            }
            None => {
                $self
                    .diagnostics
                    .push(errkind_error!(Span::default(), error_unexpected_eof!()));
                None
            }
        }
    }};
}

macro_rules! error_consume_unwrap {
    ($self:expr, $kind:ident) => {{
        match $self.parser.source_tokens.consume() {
            Some(token) => match token.kind() {
                TokenKind::$kind(inner) => Some(inner.to_owned()),
                _ => {
                    $self.diagnostics.push(errkind_error!(
                        token.span,
                        error_unexpected_token!(token.clone(), [TokenKind::$kind("".to_string())])
                    ));
                    None
                }
            },
            None => {
                $self
                    .diagnostics
                    .push(errkind_error!(Span::default(), error_unexpected_eof!()));
                None
            }
        }
    }};
}

pub struct ParserRules<'a> {
    parser: &'a mut Parser,
    diagnostics: &'a mut DiagnosticsManager,
}

impl<'a> ParserRules<'a> {
    pub fn new(parser: &'a mut Parser, diagnostics: &'a mut DiagnosticsManager) -> Self {
        ParserRules {
            parser,
            diagnostics,
        }
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let function = self.parse_function()?;

        error_expect!(self, token_eof!());

        Some(Program {
            function_definition: function,
        })
    }

    fn parse_function(&mut self) -> Option<Function> {
        error_expect!(self, token_keyword!(Int))?;
        let identifier = self.parse_identifier()?;
        error_expect!(self, token_punctuation!(OpenParen))?;
        error_expect!(self, token_keyword!(Void))?;
        error_expect!(self, token_punctuation!(CloseParen))?;
        error_expect!(self, token_punctuation!(OpenBrace))?;
        let statement = self.parse_statement()?;
        error_expect!(self, token_punctuation!(CloseBrace))?;

        Some(Function {
            identifier_name: identifier,
            statement_body: statement,
        })
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.parser.source_tokens.peek()?.kind() {
            TokenKind::Keyword(KeywordKind::Return) => {
                error_expect!(self, token_keyword!(Return))?;
                let expression = self.parse_expression()?;
                error_expect!(self, token_punctuation!(Semicolon))?;
                Some(Statement::Return { expression })
            }
            _ => {
                let token = self.parser.source_tokens.peek()?;
                self.diagnostics.push(
                    errkind_error!(token.span, error_unknown_token!(token.clone()))
                        .with(errkind_note!(token.span, "expected an statement here")),
                );
                None
            }
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        match self.parser.source_tokens.peek()?.kind() {
            TokenKind::Constant(_) => {
                let constant = self.parse_constant()?;
                Some(Expression::Constant { constant })
            }
            _ => {
                let token = self.parser.source_tokens.peek()?;
                self.diagnostics.push(
                    errkind_error!(token.span, error_unknown_token!(token.clone()))
                        .with(errkind_note!(token.span, "expected an expression here")),
                );
                None
            }
        }
    }

    fn parse_identifier(&mut self) -> Option<String> {
        error_consume_unwrap!(self, Identifier)
    }

    fn parse_constant(&mut self) -> Option<String> {
        error_consume_unwrap!(self, Constant)
    }
}
