use crate::conversion::FromGValue;
use crate::process::traversal::GraphTraversal;
use crate::structure::GValue;
use crate::structure::IntoPredicate;

pub struct WhereStep {
    params: Vec<GValue>,
}

impl WhereStep {
    fn new(params: Vec<GValue>) -> Self {
        WhereStep { params }
    }
}

impl WhereStep {
    pub fn take_params(self) -> Vec<GValue> {
        self.params
    }
}

pub trait IntoWhereStep {
    fn into_step(self) -> WhereStep;
}

impl<S, E: FromGValue> IntoWhereStep for GraphTraversal<S, E> {
    fn into_step(self) -> WhereStep {
        WhereStep::new(vec![self.bytecode.into()])
    }
}

impl<A, B> IntoWhereStep for (A, B)
where
    A: Into<String>,
    B: IntoPredicate,
{
    fn into_step(self) -> WhereStep {
        WhereStep::new(vec![self.0.into().into(), self.1.into_predicate().into()])
    }
}

impl<A> IntoWhereStep for A
where
    A: IntoPredicate,
{
    fn into_step(self) -> WhereStep {
        WhereStep::new(vec![self.into_predicate().into()])
    }
}
