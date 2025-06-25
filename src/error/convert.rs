use crate::token::*;

pub trait UserFriendlyDisplay {
    fn to_user_string(&self) -> &'static str;
}

impl UserFriendlyDisplay for TokenKind {
    fn to_user_string(&self) -> &'static str {
        match self {
            TokenKind::Keyword(kw) => kw.to_user_string(),
            TokenKind::Identifier(_) => "identifier",
            TokenKind::Operator(_) => "operator",
            TokenKind::Punctuation(punc) => punc.to_user_string(),
            TokenKind::Preprocessor(_) => "preprocessor directive",
            TokenKind::Constant(_) => "constant",
            TokenKind::Unknown(_) => "unknown token",
        }
    }
}

impl UserFriendlyDisplay for KeywordKind {
    fn to_user_string(&self) -> &'static str {
        match self {
            KeywordKind::Int => "int",
            KeywordKind::Return => "return",
            KeywordKind::Void => "void",
        }
    }
}

impl UserFriendlyDisplay for PunctuationKind {
    fn to_user_string(&self) -> &'static str {
        match self {
            PunctuationKind::Semicolon => ";",
            PunctuationKind::OpenParen => "(",
            PunctuationKind::CloseParen => ")",
            PunctuationKind::OpenBrace => "{",
            PunctuationKind::CloseBrace => "}",
        }
    }
}