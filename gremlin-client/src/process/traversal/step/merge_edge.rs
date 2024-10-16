use std::collections::HashMap;

use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;
use crate::GKey;

pub struct MergeEdgeStep {
    params: Vec<GValue>,
}

impl MergeEdgeStep {
    fn new(params: Vec<GValue>) -> Self {
        MergeEdgeStep { params }
    }
}

impl From<MergeEdgeStep> for Vec<GValue> {
    fn from(step: MergeEdgeStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for MergeEdgeStep {
    fn from(param: TraversalBuilder) -> Self {
        MergeEdgeStep::new(vec![param.bytecode.into()])
    }
}

impl From<HashMap<GKey, GValue>> for MergeEdgeStep {
    fn from(value: HashMap<GKey, GValue>) -> Self {
        MergeEdgeStep::new(vec![value.into()])
    }
}
