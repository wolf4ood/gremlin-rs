use crate::structure::GValue;

pub struct ByStep {
    params: Vec<GValue>,
}

impl ByStep {
    fn new(params: Vec<GValue>) -> Self {
        ByStep { params }
    }
}

impl ByStep {
    pub fn to_params(self) -> Vec<GValue> {
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
