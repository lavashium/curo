use common::*;
use super::*;

impl<'scp, 'ctx> Factory<(), TypedBlockItem> for LoopLabeling<'scp, 'ctx> {
    fn run(block_item: &mut TypedBlockItem, ctx: &mut AnalyzerContext<'scp, 'ctx>) -> () {
        match block_item {
            TypedBlockItem::Statement(stmt) => Self::run(stmt, ctx),
            TypedBlockItem::Declaration(_) => {},
        }
    }
}