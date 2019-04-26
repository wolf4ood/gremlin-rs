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
