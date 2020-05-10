use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct CoalesceStep {
    params: Vec<GValue>,
}

impl CoalesceStep {
    fn new(params: Vec<GValue>) -> Self {
        CoalesceStep { params }
    }
}

impl From<CoalesceStep> for Vec<GValue> {
    fn from(step: CoalesceStep) -> Self {
        step.params
    }
}

pub trait IntoCoalesceStep {
    fn into_step(self) -> CoalesceStep;
}

impl IntoCoalesceStep for TraversalBuilder {
    fn into_step(self) -> CoalesceStep {
        CoalesceStep::new(vec![self.bytecode.into()])
    }
}

impl IntoCoalesceStep for Vec<TraversalBuilder> {
    fn into_step(self) -> CoalesceStep {
        CoalesceStep::new(self.into_iter().map(|s| s.bytecode.into()).collect())
    }
}

macro_rules! impl_into_coalesce {
    ($n:expr) => {
        impl IntoCoalesceStep for [TraversalBuilder; $n] {
            fn into_step(self) -> CoalesceStep {
                CoalesceStep::new(self.iter().map(|s| s.bytecode.clone().into()).collect())
            }
        }
    };
}

impl_into_coalesce!(1);
impl_into_coalesce!(2);
impl_into_coalesce!(3);
impl_into_coalesce!(4);
impl_into_coalesce!(5);
impl_into_coalesce!(6);
impl_into_coalesce!(7);
impl_into_coalesce!(8);
impl_into_coalesce!(9);
impl_into_coalesce!(10);
