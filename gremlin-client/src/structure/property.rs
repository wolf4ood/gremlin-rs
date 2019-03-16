use crate::GValue;

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    label: String,
    value: Box<GValue>,
}

impl Property {
    pub fn new<T, GT>(label: T, value: GT) -> Property
    where
        T: Into<String>,
        GT: Into<GValue>,
    {
        Property {
            label: label.into(),
            value: Box::new(value.into()),
        }
    }
}
