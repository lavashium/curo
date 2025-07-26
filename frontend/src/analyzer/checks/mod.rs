mod resolution;
mod type_checker;
mod loop_labeling;

pub use resolution::*;
pub use type_checker::*;
pub use loop_labeling::*;

use common::*;

pub type CHECKS = factory_pipeline!(
    IdentifierResolutionCheck,
    TypeCheckCheck,
    LoopLabelingCheck,
);