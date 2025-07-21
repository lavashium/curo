use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_block_item(&mut self, item: &mut TypedBlockItem) -> Vec<TacInstruction> {
        <Self as Factory<Vec<TacInstruction>, Self, TypedBlockItem>>::run(self, item)
    }
}

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, Self, TypedBlockItem> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, item: &mut TypedBlockItem) -> Vec<TacInstruction> {
        match item {
            TypedBlockItem::Statement(stmt)   => driver.transform_statement(stmt),
            TypedBlockItem::Declaration(decl) => driver.transform_declaration(decl),
        }
    }
}