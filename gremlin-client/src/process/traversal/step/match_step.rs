use crate::process::traversal::TraversalBuilder;
use crate::structure::GValue;

pub struct MatchStep {
    params: Vec<GValue>,
}

impl MatchStep {
    fn new(params: Vec<GValue>) -> Self {
        MatchStep { params }
    }
}

impl MatchStep {
    pub fn take_params(self) -> Vec<GValue> {
        self.params
    }
}

pub trait IntoMatchStep {
    fn into_step(self) -> MatchStep;
}

impl IntoMatchStep for TraversalBuilder {
    fn into_step(self) -> MatchStep {
        MatchStep::new(vec![self.bytecode.into()])
    }
}

impl IntoMatchStep for Vec<TraversalBuilder> {
    fn into_step(self) -> MatchStep {
        MatchStep::new(self.into_iter().map(|s| s.bytecode.into()).collect())
    }
}
