use crate::structure::GValue;

pub struct DedupStep {
    params: Vec<GValue>,
}

impl DedupStep {
    fn new(params: Vec<GValue>) -> Self {
        DedupStep { params }
    }

    pub fn params(self) -> Vec<GValue> {
        self.params
    }
}

impl Into<DedupStep> for () {
    fn into(self) -> DedupStep {
        DedupStep::new(vec![])
    }
}

impl Into<DedupStep> for &str {
    fn into(self) -> DedupStep {
        DedupStep::new(vec![String::from(self).into()])
    }
}
