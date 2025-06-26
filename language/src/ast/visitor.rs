use super::node::*;

pub trait Visitor {
    fn visit_program(&mut self, program: &Program) {
        program.function_definition.accept(self);
    }

    fn visit_function(&mut self, function: &Function) {
        function.statement_body.accept(self);
    }

    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Return { expression } => {
                expression.accept(self);
            }
        }
    }

    fn visit_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Constant { constant: _ } => {}
        }
    }
}

pub trait Acceptor {
    fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V);
}

impl Acceptor for Program {
    fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        visitor.visit_program(self);
    }
}

impl Acceptor for Function {
    fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        visitor.visit_function(self);
    }
}

impl Acceptor for Statement {
    fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        visitor.visit_statement(self);
    }
}

impl Acceptor for Expression {
    fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        visitor.visit_expression(self);
    }
}
