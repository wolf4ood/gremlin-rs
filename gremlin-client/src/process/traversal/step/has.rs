use crate::structure::GValue;
use crate::structure::{IntoPredicate, P as Predicate};

pub struct HasStep {
    label: Option<String>,
    key: String,
    predicate: Option<Predicate>,
}

impl HasStep {
    pub fn take_params(self) -> Vec<GValue> {
        let mut params: Vec<GValue> = vec![];

        if let Some(s) = self.label {
            params.push(Into::into(s));
        }

        params.push(Into::into(self.key));

        if let Some(p) = self.predicate {
            params.push(Into::into(p));
        }

        params
    }
}

pub trait IntoHasStep {
    fn into_step(self) -> HasStep;
}

impl<A, B> IntoHasStep for (A, B)
where
    A: Into<String>,
    B: IntoPredicate,
{
    fn into_step(self) -> HasStep {
        HasStep {
            label: None,
            key: self.0.into(),
            predicate: Some(self.1.into_predicate()),
        }
    }
}

impl<A, B, C> IntoHasStep for (A, B, C)
where
    A: Into<String>,
    B: Into<String>,
    C: IntoPredicate,
{
    fn into_step(self) -> HasStep {
        HasStep {
            label: Some(self.0.into()),
            key: self.1.into(),
            predicate: Some(self.2.into_predicate()),
        }
    }
}

impl IntoHasStep for (String) {
    fn into_step(self) -> HasStep {
        HasStep {
            label: None,
            key: self,
            predicate: None,
        }
    }
}

impl IntoHasStep for (&str) {
    fn into_step(self) -> HasStep {
        HasStep {
            label: None,
            key: String::from(self),
            predicate: None,
        }
    }
}
