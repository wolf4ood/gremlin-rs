use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct NotStep {
    params: Vec<GValue>,
}

impl NotStep {
    fn new(params: Vec<GValue>) -> Self {
        NotStep { params }
    }
}

impl From<NotStep> for Vec<GValue> {
    fn from(step: NotStep) -> Self {
        step.params
    }
}

pub trait IntoNotStep {
    fn into_step(self) -> NotStep;
}

impl IntoNotStep for TraversalBuilder {
    fn into_step(self) -> NotStep {
        NotStep::new(vec![self.bytecode.into()])
    }
}
