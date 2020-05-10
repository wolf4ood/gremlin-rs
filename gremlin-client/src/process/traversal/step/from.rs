use crate::process::traversal::TraversalBuilder;
use crate::structure::{GValue, Vertex};

pub struct FromStep {
    params: Vec<GValue>,
}

impl FromStep {
    fn new(params: Vec<GValue>) -> Self {
        FromStep { params }
    }
}

impl From<FromStep> for Vec<GValue> {
    fn from(step: FromStep) -> Self {
        step.params
    }
}

pub trait IntoFromStep {
    fn into_step(self) -> FromStep;
}

impl IntoFromStep for &str {
    fn into_step(self) -> FromStep {
        FromStep::new(vec![self.into()])
    }
}

impl IntoFromStep for &Vertex {
    fn into_step(self) -> FromStep {
        FromStep::new(vec![self.into()])
    }
}

impl IntoFromStep for TraversalBuilder {
    fn into_step(self) -> FromStep {
        FromStep::new(vec![self.bytecode.into()])
    }
}
