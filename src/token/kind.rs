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