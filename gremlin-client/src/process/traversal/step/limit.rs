use crate::structure::{GValue, Scope};

pub struct LimitStep {
    limit: GValue,
    scope: Option<Scope>,
}

impl LimitStep {
    fn new(limit: GValue, scope: Option<Scope>) -> Self {
        LimitStep { limit, scope }
    }
}

impl LimitStep {
    pub fn params(self) -> Vec<GValue> {
        let mut params = self
            .scope
            .map(|m| match m {
                Scope::Global => vec![String::from("Global").into()],
                Scope::Local => vec![String::from("Local").into()],
            })
            .unwrap_or_else(|| vec![]);

        params.push(self.limit);
        params
    }
}

impl Into<LimitStep> for i64 {
    fn into(self) -> LimitStep {
        LimitStep::new(self.into(), None)
    }
}
