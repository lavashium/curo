use crate::*;

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

impl OperatorKind {
    pub fn to_unary(&self) -> Option<UnaryKind> {
        match self {
            OperatorKind::Minus => Some(UnaryKind::Negate),
            OperatorKind::Tilde => Some(UnaryKind::Complement),
            _ => None
        }
    }

    pub fn to_binary(&self) -> Option<BinaryKind> {
        match self {
            OperatorKind::Asterisk => Some(BinaryKind::Multiply),
            OperatorKind::ForwardSlash => Some(BinaryKind::Divide),
            OperatorKind::Minus => Some(BinaryKind::Subtract),
            OperatorKind::PercentSign => Some(BinaryKind::Remainder),
            OperatorKind::Plus => Some(BinaryKind::Add),
            _ => None,
        }
    }
}