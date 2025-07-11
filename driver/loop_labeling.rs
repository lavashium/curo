use super::*;
use language::*;
use common::*;

pub struct LoopLabelingCheck;

impl SemanticCheck for LoopLabelingCheck {
    fn analyze(ast: &mut AstProgram, ctx: &mut SemanticContext) {
        let mut labeler = LoopLabeler::new();
        ast.root = labeler.label_statement(std::mem::take(&mut ast.root), None);
    }
}

pub struct LoopLabeler {
    next_label_id: usize,
}

impl LoopLabeler {
    pub fn new() -> Self {
        Self { next_label_id: 1 }
    }

    fn make_label(&mut self) -> String {
        let label = format!("loop.{}", self.next_label_id);
        self.next_label_id += 1;
        label
    }

    pub fn label_statement(&mut self, stmt: AstStatement, current_label: Option<String>) -> AstStatement {
        use AstStatement::*;

        match stmt {
            While { condition, body, label: _, span } => {
                let new_label = self.make_label();
                let labeled_body = Box::new(self.label_statement(*body, Some(new_label.clone())));
                While {
                    condition,
                    body: labeled_body,
                    label: new_label,
                    span,
                }
            }
            DoWhile { condition, body, label: _, span } => {
                let new_label = self.make_label();
                let labeled_body = Box::new(self.label_statement(*body, Some(new_label.clone())));
                DoWhile {
                    condition,
                    body: labeled_body,
                    label: new_label,
                    span,
                }
            }
            For { for_init, condition, post, body, label: _, span } => {
                let new_label = self.make_label();
                let labeled_body = Box::new(self.label_statement(*body, Some(new_label.clone())));
                For {
                    for_init,
                    condition,
                    post,
                    body: labeled_body,
                    label: new_label,
                    span,
                }
            }
            Break { label: _, span } => {
                let label = current_label.expect("break statement outside of loop");
                Break { label, span }
            }
            Continue { label: _, span } => {
                let label = current_label.expect("continue statement outside of loop");
                Continue { label, span }
            }
            If { condition, then_branch, else_branch, span } => {
                let then_branch = Box::new(self.label_statement(*then_branch, current_label.clone()));
                let else_branch = else_branch.map(|eb| Box::new(self.label_statement(*eb, current_label.clone())));
                If {
                    condition,
                    then_branch,
                    else_branch,
                    span,
                }
            }
            Compound { block, span } => {
                let block = self.label_block(block, current_label);
                Compound { block, span }
            }
            Return { expression, span } => Return { expression, span },
            Expression { expression, span } => Expression { expression, span },
            Null => Null,
        }
    }

    fn label_block(&mut self, block: AstBlock, current_label: Option<String>) -> AstBlock {
        let block_items = block.block_items.into_iter()
            .map(|item| match item {
                AstBlockItem::Statement(stmt) => AstBlockItem::Statement(self.label_statement(stmt, current_label.clone())),
                AstBlockItem::Declaration(decl) => AstBlockItem::Declaration(decl), // declarations don't need labeling
            })
            .collect();

        AstBlock { block_items }
    }
}
