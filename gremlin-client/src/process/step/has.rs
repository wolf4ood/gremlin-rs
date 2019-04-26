use crate::structure::GValue;
use crate::structure::P as Predicate;

pub struct HasStep {
    label: Option<String>,
    key: String,
    predicate: Predicate,
}

impl HasStep {
    pub fn to_params(self) -> Vec<GValue> {
        match self.label {
            Some(s) => vec![s.into(), self.key.into(), self.predicate.into()],
            None => vec![self.key.into(), self.predicate.into()],
        }
    }
}

pub trait IntoHasStep {
    fn into(self) -> HasStep;
}

impl<A, B> IntoHasStep for (A, B)
where
    A: Into<String>,
    B: Into<Predicate>,
{
    fn into(self) -> HasStep {
        HasStep {
            label: None,
            key: self.0.into(),
            predicate: self.1.into(),
        }
    }
}

impl<A, B, C> IntoHasStep for (A, B, C)
where
    A: Into<String>,
    B: Into<String>,
    C: Into<Predicate>,
{
    fn into(self) -> HasStep {
        HasStep {
            label: Some(self.0.into()),
            key: self.1.into(),
            predicate: self.2.into(),
        }
    }
}
