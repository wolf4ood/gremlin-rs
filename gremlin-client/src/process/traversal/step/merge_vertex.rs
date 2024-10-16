use std::collections::HashMap;

use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;
use crate::GKey;

pub struct MergeVertexStep {
    params: Vec<GValue>,
}

impl MergeVertexStep {
    fn new(params: Vec<GValue>) -> Self {
        MergeVertexStep { params }
    }
}

impl From<MergeVertexStep> for Vec<GValue> {
    fn from(step: MergeVertexStep) -> Self {
        step.params
    }
}

impl From<TraversalBuilder> for MergeVertexStep {
    fn from(param: TraversalBuilder) -> Self {
        MergeVertexStep::new(vec![param.bytecode.into()])
    }
}

impl From<HashMap<GKey, GValue>> for MergeVertexStep {
    fn from(value: HashMap<GKey, GValue>) -> Self {
        MergeVertexStep::new(vec![value.into()])
    }
}
