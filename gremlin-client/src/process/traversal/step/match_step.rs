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

macro_rules! impl_into_match {
    ($n:expr) => {
        impl IntoMatchStep for [TraversalBuilder; $n] {
            fn into_step(self) -> MatchStep {
                MatchStep::new(
                    self.iter()
                        .map(|s| s.bytecode.clone().into())
                        .collect(),
                )
            }
        }
    };
}

impl_into_match!(1);
impl_into_match!(2);
impl_into_match!(3);
impl_into_match!(4);
impl_into_match!(5);
impl_into_match!(6);
impl_into_match!(7);
impl_into_match!(8);
impl_into_match!(9);
impl_into_match!(10);
