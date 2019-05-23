use crate::conversion::FromGValue;
use crate::process::traversal::remote::Terminator;
use crate::process::traversal::{GraphTraversal, Order};
use crate::structure::{GValue, T};

pub struct ByStep {
    params: Vec<GValue>,
}

impl ByStep {
    fn new(params: Vec<GValue>) -> Self {
        ByStep { params }
    }
}

impl ByStep {
    pub fn take_params(self) -> Vec<GValue> {
        self.params
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

impl<S, E: FromGValue, A> IntoByStep for (GraphTraversal<S, E, A>, Order)
where
    A: Terminator<E>,
{
    fn into_step(self) -> ByStep {
        ByStep::new(vec![self.0.bytecode.into(), self.1.into()])
    }
}
impl<S, E: FromGValue, A> IntoByStep for GraphTraversal<S, E, A>
where
    A: Terminator<E>,
{
    fn into_step(self) -> ByStep {
        ByStep::new(vec![self.bytecode.into()])
    }
}
