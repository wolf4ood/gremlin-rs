use crate::structure::{GValue, GID};
use crate::GremlinResult;

use crate::conversion::FromGValue;

#[derive(Debug, PartialEq, Clone)]
pub struct VertexProperty {
    label: String,
    id: GID,
    value: Box<GValue>,
}

impl VertexProperty {
    pub fn new<G, T, GT>(id: G, label: T, value: GT) -> VertexProperty
    where
        G: Into<GID>,
        T: Into<String>,
        GT: Into<GValue>,
    {
        VertexProperty {
            id: id.into(),
            label: label.into(),
            value: Box::new(value.into()),
        }
    }

    pub fn id(&self) -> &GID {
        &self.id
    }

    pub fn value(&self) -> &GValue {
        &self.value
    }

    pub fn take<T>(self) -> GremlinResult<T>
    where
        T: FromGValue,
    {
        T::from_gvalue(*self.value)
    }

    pub fn label(&self) -> &String {
        &self.label
    }
}
