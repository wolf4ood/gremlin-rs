use crate::{GValue, ToGValue};

#[derive(Debug, PartialEq, Clone)]
pub struct P {
    pub(crate) operator: String,
    pub(crate) value: Box<GValue>,
}

impl P {
    pub fn operator(&self) -> &String {
        &self.operator
    }

    pub fn value(&self) -> &GValue {
        &self.value
    }

    pub(crate) fn new<T>(operator: T, value: GValue) -> P
    where
        T: Into<String>,
    {
        P {
            operator: operator.into(),
            value: Box::new(value),
        }
    }
    pub fn eq<V>(value: &V) -> P
    where
        V: ToGValue,
    {
        P::new("eq", value.to_gvalue())
    }
}

impl Into<P> for &str {
    fn into(self) -> P {
        P::new("eq", (self).into())
    }
}

impl Into<P> for i32 {
    fn into(self) -> P {
        P::new("eq", (self).into())
    }
}
