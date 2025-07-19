mod resolution;
mod type_checker;
mod loop_labeling;

use resolution::*;
use type_checker::*;
use loop_labeling::*;

use common::*;

pub type CHECKS = factory_pipeline!(
    VariableResolutionCheck,
    TypeCheck,
    LoopLabelingCheck,
);