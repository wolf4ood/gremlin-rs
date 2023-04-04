use crate::{process::traversal::TraversalBuilder, GValue};

pub trait IntoSideEffectStep {
    fn into_step(self) -> Vec<GValue>;
}

impl IntoSideEffectStep for TraversalBuilder {
    fn into_step(self) -> Vec<GValue> {
        vec![self.bytecode.into()]
    }
}
