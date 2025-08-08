use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedBlock> for LoopLabeling<'scp, 'ctx> {
    fn run(block: &mut TypedBlock, ctx: &mut AnalyzerContext<'scp, 'ctx>) -> () {
        for item in block.block_items_mut() {
            LoopLabeling::run(item, ctx);
        }
    }
}