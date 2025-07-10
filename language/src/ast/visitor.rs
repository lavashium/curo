use super::node::*;

pub trait Visitor {
    fn visit_program(&mut self, program: &AstProgram) {
        program.function_definition().accept(self);
    }

    fn visit_function(&mut self, function: &AstFunction) {
        for item in function.body().block_items() {
            item.accept(self);
        }
    }

    fn visit_block_item(&mut self, item: &AstBlockItem) {
        match item {
            AstBlockItem::Statement(stmt) => stmt.accept(self),
            AstBlockItem::Declaration(decl) => decl.accept(self),
        }
    }

    fn visit_declaration(&mut self, declaration: &AstDeclaration) {
        if let Some(init) = declaration.init() {
            init.accept(self);
        }
    }

    fn visit_statement(&mut self, statement: &AstStatement) {
        match statement {
            AstStatement::Return { expression, .. } => {
                expression.accept(self);
            }
            AstStatement::Expression { expression, .. } => {
                expression.accept(self);
            }
            AstStatement::If { condition, then_branch, else_branch, .. } => {
                condition.accept(self);
                then_branch.accept(self);
                if let Some(else_expression) = else_branch {
                    else_expression.accept(self);
                }
            }
            AstStatement::Compound { block } => {
                for block in block.block_items() {
                    block.accept(self);
                }
            }
            AstStatement::Null => {}
        }
    }

    fn visit_expression(&mut self, expression: &AstExpression) {
        match expression {
            AstExpression::Constant { .. } => {}
            AstExpression::Var { .. } => {}
            AstExpression::Unary { operand, .. } => {
                operand.accept(self);
            }
            AstExpression::Binary { left, right, .. } => {
                left.accept(self);
                right.accept(self);
            }
            AstExpression::Assignment { left, right, .. } => {
                left.accept(self);
                right.accept(self);
            }
            AstExpression::Conditional { condition, then_branch, else_branch, .. } => {
                condition.accept(self);
                then_branch.accept(self);
                else_branch.accept(self);
            }
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

impl Acceptor for AstBlockItem {
    fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        visitor.visit_block_item(self);
    }
}

impl Acceptor for AstDeclaration {
    fn accept<V: Visitor + ?Sized>(&self, visitor: &mut V) {
        visitor.visit_declaration(self);
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
