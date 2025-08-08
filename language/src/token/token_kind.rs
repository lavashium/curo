use constructors::constructors;

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Keyword(KeywordKind),
    Identifier(String),
    Operator(OperatorKind),
    Punctuation(PunctuationKind),
    Constant(String),
    Unknown(String),
    Irrelevant,
    EOF,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordKind {
    Int,
    Return,
    Void,
    If,
    Else,
    Do,
    While,
    For,
    Break,
    Continue,
    Static,
    Extern,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorKind {
    Tilde,
    Minus,
    Plus,
    Asterisk,
    Slash,
    Percent,
    Exclamation,
    LessThan,
    GreaterThan,
    Equal,
    LogicalAnd,
    LogicalOr,
    EqualEqual,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Question,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PunctuationKind {
    Semicolon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Colon,
    Comma,
}

#[macro_export]
macro_rules! token_keyword {
    ($kw:ident) => {
        TokenKind::Keyword(KeywordKind::$kw)
    };
}

#[macro_export]
macro_rules! token_identifier {
    ($name:expr) => {
        TokenKind::Identifier($name.to_string())
    };
}

#[macro_export]
macro_rules! token_operator {
    ($op:ident) => {
        TokenKind::Operator(OperatorKind::$op)
    };
}

#[macro_export]
macro_rules! token_punctuation {
    ($punc:ident) => {
        TokenKind::Punctuation(PunctuationKind::$punc)
    };
}

#[macro_export]
macro_rules! token_preprocessor {
    ($text:expr) => {
        TokenKind::Preprocessor(Preprocessor($text.to_string()))
    };
}

#[macro_export]
macro_rules! token_constant {
    ($val:expr) => {
        TokenKind::Constant($val.to_string())
    };
}

#[macro_export]
macro_rules! token_unknown {
    ($text:expr) => {
        TokenKind::Unknown($text.to_string())
    };
}

#[macro_export]
macro_rules! token_eof {
    () => {
        TokenKind::EOF
    };
}

#[macro_export]
macro_rules! token_irrelevant {
    () => {
        TokenKind::Irrelevant
    };
}
