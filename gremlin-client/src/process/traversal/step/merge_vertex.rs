use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct MergeVertexStep {
    params: Vec<GValue>,
}

impl MergeVertexStep {
    fn new(params: Vec<GValue>) -> Self {
        MergeVertexStep { params }
    }
}

//todo need to handle (Map<Object,​Object> searchCreate)?
//https://tinkerpop.apache.org/docs/current/dev/provider/#merge-v-step
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
