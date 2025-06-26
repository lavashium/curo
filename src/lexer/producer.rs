use crate::*;
use crate::lexer::Lexer;
use crate::token::*;

macro_rules! auto_nest {
    () => {
        ()
    };
    ($head:ty $(, $tail:ty)* $(,)?) => {
        ($head, auto_nest!($($tail),*))
    };
}

pub trait TokenProducer {
    fn try_match(lexer: &mut Lexer) -> Option<Token>;
}

pub trait TokenProducerList {
    fn try_all(lexer: &mut Lexer) -> Option<Token>;
}

impl TokenProducerList for () {
    fn try_all(_: &mut Lexer) -> Option<Token> {
        None
    }
}

impl<Head: TokenProducer, Tail: TokenProducerList> TokenProducerList for (Head, Tail) {
    fn try_all(lexer: &mut Lexer) -> Option<Token> {
        Head::try_match(lexer).or_else(|| Tail::try_all(lexer))
    }
}

pub type PRODUCERS = auto_nest!(
    WhitespaceProducer,
    KeywordProducer,
    ConstantProducer,
    PunctuationProducer,
);

pub struct WhitespaceProducer;
pub struct KeywordProducer;
pub struct PunctuationProducer;
pub struct ConstantProducer;

impl TokenProducer for WhitespaceProducer {
    fn try_match(lexer: &mut Lexer) -> Option<Token> {
        let ch = lexer.peek()?;
        if !ch.is_whitespace() {
            return None;
        }

        let start_pos = lexer.current_position();

        while let Some(c) = lexer.peek() {
            if c.is_whitespace() {
                lexer.advance();
            } else {
                break;
            }
        }

        let span = lexer.span_from(start_pos);

        Some(Token::new(
            token_unknown!("Whitespace"),
            " ".to_string(),
            span,
        ))
    }
}

impl TokenProducer for KeywordProducer {
    fn try_match(lexer: &mut Lexer) -> Option<Token> {
        let start_pos = lexer.current_position();
        let start_ptr = lexer.pointer();

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

        let end_ptr = lexer.pointer();
        let lexeme = lexer.peek_slice((start_ptr, end_ptr))?.to_string();

        let kind = match lexeme.as_str() {
            "int"    => token_keyword!(Int),
            "return" => token_keyword!(Return),
            "void"   => token_keyword!(Void),
            _        => TokenKind::Identifier(lexeme.clone()),
        };

        let span = lexer.span_from(start_pos);

        Some(Token::new(
            kind,
            lexeme,
            span
        ))
    }
}

impl TokenProducer for PunctuationProducer {
    fn try_match(lexer: &mut Lexer) -> Option<Token> {
        let ch = lexer.peek()?;

        let kind = match ch {
            ';' => token_punctuation!(Semicolon),
            '(' => token_punctuation!(OpenParen),
            ')' => token_punctuation!(CloseParen),
            '{' => token_punctuation!(OpenBrace),
            '}' => token_punctuation!(CloseBrace),
            _ => return None,
        };

        let start_pos = lexer.current_position();

        lexer.advance();

        let span = lexer.span_from(start_pos);
        let lexeme = ch.to_string();

        Some(Token::new(
            kind,
            lexeme,
            span
        ))
    }
}

impl TokenProducer for ConstantProducer {
    fn try_match(lexer: &mut Lexer) -> Option<Token> {
        let start_pos = lexer.current_position();
        let start_ptr = lexer.pointer();

        let ch = lexer.peek()?;
        if !ch.is_ascii_digit() {
            return None;
        }

        while let Some(c) = lexer.peek() {
            if c.is_ascii_digit() {
                lexer.advance();
            } else {
                break;
            }
        }

        let end_ptr = lexer.pointer();
        let lexeme = lexer.peek_slice((start_ptr, end_ptr))?.to_string();

        let kind = TokenKind::Constant(lexeme.clone());

        let span = lexer.span_from(start_pos);

        Some(Token::new(
            kind,
            lexeme,
            span
        ))
    }
}



