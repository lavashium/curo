use constructors::constructors;

#[constructors]
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
