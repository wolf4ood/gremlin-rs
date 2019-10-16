use crate::structure::GValue;
use crate::structure::T;
use crate::structure::{IntoPredicate, P as Predicate};

pub enum HasStepKey {
    STRING(String),
    T(T),
}

impl Into<HasStepKey> for T {
    fn into(self) -> HasStepKey {
        HasStepKey::T(self)
    }
}

impl Into<HasStepKey> for String {
    fn into(self) -> HasStepKey {
        HasStepKey::STRING(self)
    }
}

impl Into<HasStepKey> for &str {
    fn into(self) -> HasStepKey {
        HasStepKey::STRING(String::from(self))
    }
}

pub struct HasStep {
    label: Option<String>,
    key: HasStepKey,
    predicate: Option<Predicate>,
}

impl HasStep {
    pub fn take_params(self) -> Vec<GValue> {
        let mut params: Vec<GValue> = vec![];

        if let Some(s) = self.label {
            params.push(Into::into(s));
        }

        match self.key {
            HasStepKey::STRING(key) => params.push(Into::into(key)),
            HasStepKey::T(key) => params.push(Into::into(key)),
        };

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
    A: Into<HasStepKey>,
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
    B: Into<HasStepKey>,
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
            key: HasStepKey::STRING(self),
            predicate: None,
        }
    }
}

impl IntoHasStep for (&str) {
    fn into_step(self) -> HasStep {
        HasStep {
            label: None,
            key: HasStepKey::STRING(String::from(self)),
            predicate: None,
        }
    }
}
