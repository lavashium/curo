use crate::*;
use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> Factory<Vec<TacInstruction>, TypedBlockItem> for GeneratorTransforms<'scp, 'ctx> {
    fn run(item: &mut TypedBlockItem, ctx: &mut TacGenContext<'scp, 'ctx>) -> Vec<TacInstruction> {
        match item {
            TypedBlockItem::Statement(stmt)   => Self::run(stmt, ctx),
            TypedBlockItem::Declaration(decl) => Self::run(decl, ctx),
        }
    }
}