use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct OrStep {
    params: Vec<GValue>,
}

impl OrStep {
    fn new(params: Vec<GValue>) -> Self {
        OrStep { params }
    }
}

impl From<OrStep> for Vec<GValue> {
    fn from(step: OrStep) -> Self {
        step.params
    }
}

pub trait IntoOrStep {
    fn into_step(self) -> OrStep;
}

impl IntoOrStep for () {
    fn into_step(self) -> OrStep {
        OrStep::new(vec![])
    }
}

impl IntoOrStep for TraversalBuilder {
    fn into_step(self) -> OrStep {
        OrStep::new(vec![self.bytecode.into()])
    }
}

impl IntoOrStep for Vec<TraversalBuilder> {
    fn into_step(self) -> OrStep {
        OrStep::new(self.into_iter().map(|s| s.bytecode.into()).collect())
    }
}

macro_rules! impl_into_or {
    ($n:expr) => {
        impl IntoOrStep for [TraversalBuilder; $n] {
            fn into_step(self) -> OrStep {
                OrStep::new(self.iter().map(|s| s.bytecode.clone().into()).collect())
            }
        }
    };
}

impl_into_or!(1);
impl_into_or!(2);
impl_into_or!(3);
impl_into_or!(4);
impl_into_or!(5);
impl_into_or!(6);
impl_into_or!(7);
impl_into_or!(8);
impl_into_or!(9);
impl_into_or!(10);
