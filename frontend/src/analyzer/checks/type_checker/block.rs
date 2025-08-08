use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedBlock> for TypeCheck<'scp, 'ctx> {
    fn run(block: &mut TypedBlock, ctx: &mut AnalyzerContext<'scp, 'ctx>) {
        for item in block.block_items_mut() {
            Self::run(item, ctx);
        }
    }
}