use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstProgram {
    function_definition: AstFunction,
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstFunction {
    name: String,
    body: Vec<AstBlockItem>,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstBlockItem {
    Statement(AstStatement),
    Declaration(AstDeclaration),
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstDeclaration {
    name: String,
    init: Option<AstExpression>,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstStatement {
    Return { expression: AstExpression },
    Expression { expression: AstExpression },
    Null,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstExpression {
    Constant {
        constant: String,
    },
    Var {
        identifier: String,
    },
    Unary {
        operator: AstUnaryKind,
        operand: Box<AstExpression>,
    },
    Binary {
        operator: AstBinaryKind,
        left: Box<AstExpression>,
        right: Box<AstExpression>,
    },
    Assignment {
        left: Box<AstExpression>,
        right: Box<AstExpression>,
    },
}

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
