use crate::process::traversal::{Order, TraversalBuilder};
use crate::structure::{GValue, T};

pub struct ByStep {
    params: Vec<GValue>,
}

impl ByStep {
    fn new(params: Vec<GValue>) -> Self {
        ByStep { params }
    }
}

impl From<ByStep> for Vec<GValue> {
    fn from(step: ByStep) -> Self {
        step.params
    }
}

pub trait IntoByStep {
    fn into_step(self) -> ByStep;
}

impl IntoByStep for () {
    fn into_step(self) -> ByStep {
        ByStep::new(vec![])
    }
}

impl IntoByStep for &str {
    fn into_step(self) -> ByStep {
        ByStep::new(vec![String::from(self).into()])
    }
}

impl IntoByStep for Order {
    fn into_step(self) -> ByStep {
        ByStep::new(vec![self.into()])
    }
}

impl IntoByStep for T {
    fn into_step(self) -> ByStep {
        ByStep::new(vec![self.into()])
    }
}

impl<'a> IntoByStep for (&str, Order) {
    fn into_step(self) -> ByStep {
        ByStep::new(vec![self.0.into(), self.1.into()])
    }
}

impl IntoByStep for (String, Order) {
    fn into_step(self) -> ByStep {
        ByStep::new(vec![self.0.into(), self.1.into()])
    }
}

impl IntoByStep for (TraversalBuilder, Order) {
    fn into_step(self) -> ByStep {
        ByStep::new(vec![self.0.bytecode.into(), self.1.into()])
    }
}
impl IntoByStep for TraversalBuilder {
    fn into_step(self) -> ByStep {
        ByStep::new(vec![self.bytecode.into()])
    }
}
