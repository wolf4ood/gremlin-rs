use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct UntilStep {
    params: Vec<GValue>,
}

impl UntilStep {
    fn new(params: Vec<GValue>) -> Self {
        UntilStep { params }
    }
}

impl From<UntilStep> for Vec<GValue> {
    fn from(step: UntilStep) -> Self {
        step.params
    }
}

pub trait IntoUntilStep {
    fn into_step(self) -> UntilStep;
}

impl IntoUntilStep for TraversalBuilder {
    fn into_step(self) -> UntilStep {
        UntilStep::new(vec![self.bytecode.into()])
    }
}
