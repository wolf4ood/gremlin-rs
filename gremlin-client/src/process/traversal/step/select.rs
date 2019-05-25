use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct SelectStep {
    params: Vec<GValue>,
}

impl SelectStep {
    fn new(params: Vec<GValue>) -> Self {
        SelectStep { params }
    }
}

impl SelectStep {
    pub fn take_params(self) -> Vec<GValue> {
        self.params
    }
}

pub trait IntoSelectStep {
    fn into_step(self) -> SelectStep;
}

impl IntoSelectStep for &str {
    fn into_step(self) -> SelectStep {
        SelectStep::new(vec![String::from(self).into()])
    }
}

impl IntoSelectStep for Vec<&str> {
    fn into_step(self) -> SelectStep {
        SelectStep::new(self.into_iter().map(GValue::from).collect())
    }
}

impl IntoSelectStep for TraversalBuilder {
    fn into_step(self) -> SelectStep {
        SelectStep::new(vec![self.bytecode.into()])
    }
}
