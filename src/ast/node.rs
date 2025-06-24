#[derive(Debug)]
pub struct Program {
    pub function_definition: Function,
}

#[derive(Debug)]
pub struct Function {
    pub identifier_name: String,
    pub statement_body: Statement,
}

#[derive(Debug)]
pub enum Statement {
    Return {
        expression: Expression,
    },
}

#[derive(Debug)]
pub enum Expression {
    Constant {
        constant: String
    },
}

#[macro_export]
macro_rules! ast_expression_constant {
    ($value:expr) => {
        Expression::Constant {
            constant: $value.to_string(),
        }
    };
}

#[macro_export]
macro_rules! ast_statement_return {
    ($expr:expr) => {
        Statement::Return {
            expression: $expr,
        }
    };
}

#[macro_export]
macro_rules! ast_function {
    ($name:expr, $stmt:expr) => {
        Function {
            identifier_name: $name.to_string(),
            statement_body: $stmt,
        }
    };
}

#[macro_export]
macro_rules! ast_program {
    ($func:expr) => {
        Program {
            function_definition: $func,
        }
    };
}