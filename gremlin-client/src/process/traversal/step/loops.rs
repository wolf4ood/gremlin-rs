use crate::structure::GValue;

pub struct LoopsStep {
    params: Vec<GValue>,
}

impl LoopsStep {
    fn new(params: Vec<GValue>) -> Self {
        LoopsStep { params }
    }

    pub fn params(self) -> Vec<GValue> {
        self.params
    }
}

impl Into<LoopsStep> for () {
    fn into(self) -> LoopsStep {
        LoopsStep::new(vec![])
    }
}

impl Into<LoopsStep> for &str {
    fn into(self) -> LoopsStep {
        LoopsStep::new(vec![String::from(self).into()])
    }
}

impl Into<LoopsStep> for String {
    fn into(self) -> LoopsStep {
        LoopsStep::new(vec![self.into()])
    }
}
