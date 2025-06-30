#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstProgram {
    pub function_definition: AstFunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstFunction {
    pub name: String,
    pub body: AstStatement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstStatement {
    Return { expression: AstExpression },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstExpression {
    Constant { constant: String },
    Unary {
        operator: UnaryKind,
        operand: Box<AstExpression>,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryKind {
    Complement,
    Negation,
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
            identifier_name: $name.to_string(),
            statement_body: $stmt,
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
