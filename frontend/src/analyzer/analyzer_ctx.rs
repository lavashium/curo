use std::collections::HashMap;
use common::*;
use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
pub struct AnalyzerContext<'scp, 'ctx> {
    pub ctx: &'scp mut CompilerContext<'ctx>,
    pub scope: IdentifierMap,
    pub loop_depth: usize,
    pub inside_function: bool,
    pub global_scope: bool,
    pub current_loop: Option<String>
}

impl<'scp, 'ctx> Context for AnalyzerContext<'scp, 'ctx> {}

pub type IdentifierMap = HashMap<String, IdentifierInfo>;

#[accessors]
#[constructors]
#[derive(Clone)]
pub struct IdentifierInfo {
    pub unique_name: String,
    pub has_linkage: bool,
    pub from_current_scope: bool,
}

pub fn copy_identifier_map(map: &IdentifierMap) -> IdentifierMap {
    map.iter()
        .map(|(k, v)| (k.clone(), IdentifierInfo {
            unique_name: v.unique_name.clone(),
            has_linkage: v.has_linkage,
            from_current_scope: false,
        }))
        .collect()
}