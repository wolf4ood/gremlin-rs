use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct RepeatStep {
    params: Vec<GValue>,
}

impl RepeatStep {
    fn new(params: Vec<GValue>) -> Self {
        RepeatStep { params }
    }
}

impl RepeatStep {
    pub fn take_params(self) -> Vec<GValue> {
        self.params
    }
}

pub trait IntoRepeatStep {
    fn into_step(self) -> RepeatStep;
}

impl IntoRepeatStep for TraversalBuilder {
    fn into_step(self) -> RepeatStep {
        RepeatStep::new(vec![self.bytecode.into()])
    }
}
