use crate::conversion::FromGValue;
use crate::process::traversal::remote::Terminator;
use crate::process::traversal::GraphTraversal;
use crate::structure::GValue;

pub struct NotStep {
    params: Vec<GValue>,
}

impl NotStep {
    fn new(params: Vec<GValue>) -> Self {
        NotStep { params }
    }
}

impl NotStep {
    pub fn take_params(self) -> Vec<GValue> {
        self.params
    }
}

pub trait IntoNotStep {
    fn into_step(self) -> NotStep;
}

impl<S, E: FromGValue, A> IntoNotStep for GraphTraversal<S, E, A>
where
    A: Terminator<E>,
{
    fn into_step(self) -> NotStep {
        NotStep::new(vec![self.bytecode.into()])
    }
}
