use super::*;
use language::*;

pub trait BlockTransform {
    fn transform_block(&mut self, block: &AstBlock) -> Vec<TacInstruction>;
}

impl<'a> BlockTransform for GeneratorTransforms<'a> {
    fn transform_block(&mut self, block: &AstBlock) -> Vec<TacInstruction> {
        let mut instructions = vec![];
        for item in block.block_items() {
            instructions.append(&mut self.transform_block_item(item));
        }
        instructions
    }
}
