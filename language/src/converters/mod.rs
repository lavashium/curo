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
    pub fn to_unary(&self) -> Option<AstUnaryKind> {
        match self {
            OperatorKind::Minus       => Some(AstUnaryKind::Negate),
            OperatorKind::Tilde       => Some(AstUnaryKind::Complement),
            OperatorKind::Exclamation => Some(AstUnaryKind::Not),
            _ => None,
        }
    }

    pub fn to_binary(&self) -> Option<AstBinaryKind> {
        match self {
            OperatorKind::Plus => Some(AstBinaryKind::Add),
            OperatorKind::Minus => Some(AstBinaryKind::Subtract),
            OperatorKind::Asterisk => Some(AstBinaryKind::Multiply),
            OperatorKind::Slash => Some(AstBinaryKind::Divide),
            OperatorKind::Percent => Some(AstBinaryKind::Remainder),
            OperatorKind::LogicalAnd => Some(AstBinaryKind::And),
            OperatorKind::EqualEqual => Some(AstBinaryKind::Equal),
            OperatorKind::NotEqual => Some(AstBinaryKind::NotEqual),
            OperatorKind::LessThan => Some(AstBinaryKind::LessThan),
            OperatorKind::LessEqual => Some(AstBinaryKind::LessOrEqual),
            OperatorKind::GreaterThan => Some(AstBinaryKind::GreaterThan),
            OperatorKind::GreaterEqual => Some(AstBinaryKind::GreaterOrEqual),
            OperatorKind::LogicalOr => Some(AstBinaryKind::Or),
            OperatorKind::Tilde |
            OperatorKind::Exclamation => None
        }
    }
}

impl AstBinaryKind {
    pub fn to_tac(&self) -> Option<TacBinaryKind> {
        match self {
            AstBinaryKind::Add => Some(TacBinaryKind::Add),
            AstBinaryKind::Subtract => Some(TacBinaryKind::Subtract),
            AstBinaryKind::Multiply => Some(TacBinaryKind::Multiply),
            AstBinaryKind::Divide => Some(TacBinaryKind::Divide),
            AstBinaryKind::Remainder => Some(TacBinaryKind::Remainder),
            AstBinaryKind::And => Some(TacBinaryKind::Add),
            AstBinaryKind::Equal => Some(TacBinaryKind::Equal),
            AstBinaryKind::NotEqual => Some(TacBinaryKind::NotEqual),
            AstBinaryKind::LessThan => Some(TacBinaryKind::LessThan),
            AstBinaryKind::LessOrEqual => Some(TacBinaryKind::LessOrEqual),
            AstBinaryKind::GreaterThan => Some(TacBinaryKind::GreaterThan),
            AstBinaryKind::GreaterOrEqual => Some(TacBinaryKind::GreaterOrEqual),
            AstBinaryKind::Or => None,
        }
    }
}

impl AstUnaryKind {
    pub fn to_tac(&self) -> Option<TacUnaryKind> {
        match self {
            AstUnaryKind::Complement => Some(TacUnaryKind::Complement),
            AstUnaryKind::Negate => Some(TacUnaryKind::Negate),
            AstUnaryKind::Not => Some(TacUnaryKind::Not),
        }
    }
}