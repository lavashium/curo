use super::node::*;

pub trait Visitor {
    fn visit_program(&mut self, program: &AstProgram) {
        program.function_definition.accept(self);
    }

    fn visit_function(&mut self, function: &AstFunction) {
        function.body.accept(self);
    }

    fn visit_statement(&mut self, statement: &AstStatement) {
        match statement {
            AstStatement::Return { expression } => {
                expression.accept(self);
            }
        }
    }

    fn visit_expression(&mut self, expression: &AstExpression) {
        match expression {
            AstExpression::Constant { constant: _ } => {}
        }
    }
}

pub trait Acceptor {
    fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V);
}

impl Acceptor for AstProgram {
    fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        visitor.visit_program(self);
    }
}

impl Acceptor for AstFunction {
    fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        visitor.visit_function(self);
    }
}

impl Acceptor for AstStatement {
    fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        visitor.visit_statement(self);
    }
}

impl Acceptor for AstExpression {
    fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        visitor.visit_expression(self);
    }
}
