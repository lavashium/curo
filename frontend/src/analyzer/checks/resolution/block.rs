use common::*;
use super::*;

impl Factory<(), TypedBlock, AnalyzerContext<'_, '_>> for IdentifierResolution {
    fn run(block: &mut TypedBlock, ctx: &mut AnalyzerContext) {
        let old_scope = ctx.scope.clone();
        ctx.scope = copy_identifier_map(&ctx.scope);
    
        for item in block.block_items_mut() {
            Self::run(item, ctx);
        }

        ctx.scope = old_scope
    }
}