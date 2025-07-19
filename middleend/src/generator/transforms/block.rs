use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_block(&mut self, block: &mut AstBlock) -> Vec<TacInstruction> {
        <Self as Factory<Vec<TacInstruction>, Self, AstBlock>>::run(self, block)
    }
}

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, Self, AstBlock> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, block: &mut AstBlock) -> Vec<TacInstruction> {
        let mut instructions = vec![];
        for item in block.block_items_mut() {
            instructions.append(&mut driver.transform_block_item(item));
        }
        instructions
    }
}