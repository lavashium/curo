use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_block_item(&mut self, item: &mut AstBlockItem) -> Vec<TacInstruction> {
        <Self as Factory<Vec<TacInstruction>, Self, AstBlockItem>>::run(self, item)
    }
}

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, Self, AstBlockItem> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, item: &mut AstBlockItem) -> Vec<TacInstruction> {
        match item {
            AstBlockItem::Statement(stmt)   => driver.transform_statement(stmt),
            AstBlockItem::Declaration(decl) => driver.transform_declaration(decl),
        }
    }
}