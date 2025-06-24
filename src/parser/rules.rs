use crate::parser::Parser;
use crate::token::*;
use crate::ast::*;
use crate::*;

macro_rules! expect {
    ($parser:expr, $kind:expr) => {
        $parser.source_tokens.consume_expected($kind)?
    };
}

macro_rules! consume_unwrap_contents {
    ($parser:expr, $kind:ident) => {{
        match $parser.source_tokens.consume()?.kind() {
            TokenKind::$kind(inner) => Some(inner.to_owned()),
            _ => None,
        }
    }};
}

pub fn parse_program(parser: &mut Parser) -> Option<Program> {
    let function = parse_function(parser)?;

    if parser.source_tokens.any_tokens() {
        return None;
    }

    Some(Program {
        function_definition: function
    })
}

fn parse_function(parser: &mut Parser) -> Option<Function> {
    expect!(parser, token_keyword!(Int));

    let identifier = parse_identifier(parser)?;

    expect!(parser, token_punctuation!(OpenParen));
    expect!(parser, token_keyword!(Void));
    expect!(parser, token_punctuation!(CloseParen));
    expect!(parser, token_punctuation!(OpenBrace));

    let statement = parse_statement(parser)?;

    expect!(parser, token_punctuation!(CloseBrace));

    Some(Function {
        identifier_name: identifier,
        statement_body:  statement
    })
}

fn parse_statement(parser: &mut Parser) -> Option<Statement> {
    match parser.source_tokens.peek()?.kind() {

        TokenKind::Keyword(KeywordKind::Return) => {
            expect!(parser, token_keyword!(Return));
            let expression = parse_expression(parser)?;
            expect!(parser, token_punctuation!(Semicolon));
            Some(Statement::Return {
                expression: expression 
            })
        },

        _ => None,
    }
}

fn parse_expression(parser: &mut Parser) -> Option<Expression> {
    match parser.source_tokens.peek()?.kind() {

        TokenKind::Constant(_) => {
            let constant = parse_constant(parser)?;
            Some(Expression::Constant {
                constant: constant
            })
        }

        _ => None
    }
}

fn parse_identifier(parser: &mut Parser) -> Option<String> {
    consume_unwrap_contents!(parser, Identifier)
}

fn parse_constant(parser: &mut Parser) -> Option<String> {
    consume_unwrap_contents!(parser, Constant)
}