use super::*;
use language::*;

pub trait BlockItemTransform {
    fn transform_block_item(&mut self, item: &AstBlockItem) -> Vec<TacInstruction>;
}

impl<'a> BlockItemTransform for GeneratorTransforms<'a> {
    fn transform_block_item(&mut self, item: &AstBlockItem) -> Vec<TacInstruction> {
        match item {
            AstBlockItem::Statement(stmt) => self.transform_statement(stmt),
            AstBlockItem::Declaration(decl) => self.transform_declaration(decl),
        }
    }
}
