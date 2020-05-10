use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct LocalStep {
    params: Vec<GValue>,
}

impl LocalStep {
    fn new(params: Vec<GValue>) -> Self {
        LocalStep { params }
    }
}

impl From<LocalStep> for Vec<GValue> {
    fn from(step: LocalStep) -> Self {
        step.params
    }
}

pub trait IntoLocalStep {
    fn into_step(self) -> LocalStep;
}

impl IntoLocalStep for TraversalBuilder {
    fn into_step(self) -> LocalStep {
        LocalStep::new(vec![self.bytecode.into()])
    }
}
