use crate::process::traversal::TraversalBuilder;
use crate::structure::{GValue, Vertex};

pub struct ToStep {
    params: Vec<GValue>,
}

impl ToStep {
    fn new(params: Vec<GValue>) -> Self {
        ToStep { params }
    }
}

impl From<ToStep> for Vec<GValue> {
    fn from(step: ToStep) -> Self {
        step.params
    }
}

pub trait IntoToStep {
    fn into_step(self) -> ToStep;
}

impl IntoToStep for &str {
    fn into_step(self) -> ToStep {
        ToStep::new(vec![self.into()])
    }
}

impl IntoToStep for &Vertex {
    fn into_step(self) -> ToStep {
        ToStep::new(vec![self.into()])
    }
}

impl IntoToStep for TraversalBuilder {
    fn into_step(self) -> ToStep {
        ToStep::new(vec![self.bytecode.into()])
    }
}
