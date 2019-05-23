use crate::conversion::FromGValue;
use crate::process::traversal::remote::Terminator;
use crate::process::traversal::GraphTraversal;
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

impl<S, E: FromGValue, A> IntoSelectStep for GraphTraversal<S, E, A>
where
    A: Terminator<E>,
{
    fn into_step(self) -> SelectStep {
        SelectStep::new(vec![self.bytecode.into()])
    }
}
