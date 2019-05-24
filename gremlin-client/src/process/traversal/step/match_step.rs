use crate::conversion::FromGValue;
use crate::process::traversal::remote::Terminator;
use crate::process::traversal::GraphTraversal;
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

impl<S, E: FromGValue, A> IntoMatchStep for GraphTraversal<S, E, A>
where
    A: Terminator<E>,
{
    fn into_step(self) -> MatchStep {
        MatchStep::new(vec![self.bytecode.into()])
    }
}

impl<S, E: FromGValue, A> IntoMatchStep for Vec<GraphTraversal<S, E, A>>
where
    A: Terminator<E>,
{
    fn into_step(self) -> MatchStep {
        MatchStep::new(self.into_iter().map(|s| s.bytecode.into()).collect())
    }
}
