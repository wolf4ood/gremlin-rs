use crate::{process::traversal::TraversalBuilder, GValue};

pub struct SideEffectStep {
    params: Vec<GValue>,
}

impl SideEffectStep {
    fn new(params: Vec<GValue>) -> Self {
        SideEffectStep { params }
    }
}

impl From<SideEffectStep> for Vec<GValue> {
    fn from(step: SideEffectStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for SideEffectStep {
    fn from(param: TraversalBuilder) -> Self {
        SideEffectStep::new(vec![param.bytecode.into()])
    }
}
