use crate::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericKind {
    Keyword,
    Identifier,
    Operator,
    Punctuation,
    Constant,
    Unknown,
    EOF,
    Statement,
    Expression,
}

impl TokenKind {
    pub fn to_generic(&self) -> GenericKind {
        match self {
            TokenKind::Keyword(_) => GenericKind::Keyword,
            TokenKind::Identifier(_) => GenericKind::Identifier,
            TokenKind::Operator(_) => GenericKind::Operator,
            TokenKind::Punctuation(_) => GenericKind::Punctuation,
            TokenKind::Constant(_) => GenericKind::Constant,
            TokenKind::Unknown(_) => GenericKind::Unknown,
            TokenKind::EOF => GenericKind::EOF,
            TokenKind::Irrelevant => GenericKind::Unknown,
        }
    }
}
