#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Preprocessor(pub String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constant(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Keyword(KeywordKind),
    Identifier(Identifier),
    Operator(OperatorKind),
    Punctuation(PunctuationKind),
    Preprocessor(Preprocessor),
    Constant(Constant),
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordKind {
    Int,
    Return,
    Void,    
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorKind {
    //curently not used
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PunctuationKind {
    Semicolon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
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
        TokenKind::Identifier(Identifier($name.to_string()))
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
        TokenKind::Constant(Constant($val.to_string()))
    };
}

#[macro_export]
macro_rules! token_unknown {
    ($text:expr) => {
        TokenKind::Unknown($text.to_string())
    };
}