use constructors::constructors;

#[constructors]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AstUnaryKind {
    Complement,
    Negate,
    Not,
}

#[constructors]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AstBinaryKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
}

#[constructors]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum AstType {
    #[default]
    Int,
    FunType(usize)
}