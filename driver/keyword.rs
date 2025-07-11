use crate::lexer::Lexer;
use crate::lexer::producer::TokenProducer;
use common::*;
use language::*;

pub struct KeywordProducer;

impl TokenProducer for KeywordProducer {
    fn try_match(lexer: &mut Lexer, _diagnostics: &mut DiagnosticsManager) -> Option<Token> {
        let start_pos = lexer.current_position();
        let start_ptr = lexer.get_pointer();

        let ch = lexer.peek()?;
        if !ch.is_ascii_alphabetic() && ch != '_' {
            return None;
        }

        while let Some(c) = lexer.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                lexer.advance();
            } else {
                break;
            }
        }

        let end_ptr = lexer.get_pointer();
        let lexeme = lexer.peek_slice((start_ptr, end_ptr))?.to_string();

        let kind = lookup_keyword(&lexeme);

        let span = lexer.span_from(start_pos);

        Some(Token::new(kind, lexeme, span))
    }
}

macro_rules! keyword_table {
    ($($kw:literal => $variant:ident),* $(,)?) => {
        &[
            $(
                ($kw, token_keyword!($variant)),
            )*
        ]
    };
}

const KEYWORDS: &[(&str, TokenKind)] = keyword_table!(
    "int" => Int,
    "void" => Void,
    "return" => Return,
    "if" => If,
    "else" => Else,
    "do" => Do,
    "while" => While,
    "for" => For,
    "break" => Break,
    "continue" => Continue
);

fn lookup_keyword(candidate: &str) -> TokenKind {
    if candidate.starts_with(|c: char| c.is_ascii_digit()) {
        return TokenKind::Unknown(candidate.to_string());
    }

    for &(kw, ref kind) in KEYWORDS {
        if candidate == kw {
            return kind.clone();
        }
    }

    TokenKind::Identifier(candidate.to_string())
}
