use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstProgram {
    pub function_definition: AstFunction,
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstFunction {
    pub name: String,
    pub body: AstStatement,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstStatement {
    Return { expression: AstExpression },
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstExpression {
    Constant {
        constant: String,
    },
    Unary {
        operator: UnaryKind,
        operand: Box<AstExpression>,
    },
    Binary {
        operator: BinaryKind,
        left: Box<AstExpression>,
        right: Box<AstExpression>,
    },
}

#[constructors]
#[derive(Debug, Copy, Clone, PartialEq, Eq,)]
pub enum UnaryKind {
    Complement,
    Negate,
}

#[constructors]
#[derive(Debug, Copy, Clone, PartialEq, Eq,)]
pub enum BinaryKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

#[macro_export]
macro_rules! ast_expression_constant {
    ($value:expr) => {
        AstExpression::Constant {
            constant: $value.to_string(),
        }
    };
}

#[macro_export]
macro_rules! ast_statement_return {
    ($expr:expr) => {
        AstStatement::Return { expression: $expr }
    };
}

#[macro_export]
macro_rules! ast_function {
    ($name:expr, $stmt:expr) => {
        AstFunction {
            name: $name.to_string(),
            body: $stmt,
        }
    };
}

#[macro_export]
macro_rules! ast_program {
    ($func:expr) => {
        AstProgram {
            function_definition: $func,
        }
    };
}
