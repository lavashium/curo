use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedBlock> for IdentifierResolution<'scp, 'ctx>  {
    fn run(block: &mut TypedBlock, ctx: &mut AnalyzerContext<'scp, 'ctx> ) {
        let old_scope = ctx.scope.clone();
        ctx.scope = copy_identifier_map(&ctx.scope);
    
        for item in block.block_items_mut() {
            Self::run(item, ctx);
        }

        ctx.scope = old_scope
    }
}