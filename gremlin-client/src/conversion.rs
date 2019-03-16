use crate::{Edge, GValue, GremlinError, GremlinResult, List, Map, Vertex, VertexProperty, GID};

use std::collections::HashMap;

pub trait ToGValue {
    fn to_gvalue(&self) -> GValue;
}

#[derive(Debug, PartialEq)]
pub struct Params(pub HashMap<String, GValue>);

impl Into<Params> for () {
    fn into(self) -> Params {
        Params(HashMap::new())
    }
}

impl ToGValue for GID {
    fn to_gvalue(&self) -> GValue {
        match self {
            GID::Int32(n) => GValue::from(*n),
            GID::Int64(n) => GValue::from(*n),
            GID::String(n) => GValue::from(n.clone()),
        }
    }
}

macro_rules! impl_to_galue {
    ($t:ty, $v:path) => {
        impl ToGValue for $t {
            fn to_gvalue(&self) -> GValue {
                $v(*self)
            }
        }
    };
}

impl_to_galue!(f32, GValue::Float);
impl_to_galue!(f64, GValue::Double);
impl_to_galue!(i32, GValue::Int32);
impl_to_galue!(i64, GValue::Int64);

impl ToGValue for &str {
    fn to_gvalue(&self) -> GValue {
        GValue::String(String::from(*self))
    }
}

#[doc(hidden)]
pub trait FromGValue: Sized {
    fn from_gvalue(v: GValue) -> GremlinResult<Self>;
}

macro_rules! impl_from_galue {
    ($t:ty, $v:path) => {
        impl FromGValue for $t {
            fn from_gvalue(v: GValue) -> GremlinResult<$t> {
                match v {
                    $v(e) => Ok(e),
                    _ => Err(GremlinError::Cast(format!(
                        "Cannot convert {:?} to {}",
                        v,
                        stringify!($t)
                    ))),
                }
            }
        }
    };
}

impl_from_galue!(VertexProperty, GValue::VertexProperty);
impl_from_galue!(Map, GValue::Map);
impl_from_galue!(List, GValue::List);
impl_from_galue!(Vertex, GValue::Vertex);
impl_from_galue!(Edge, GValue::Edge);
impl_from_galue!(String, GValue::String);
impl_from_galue!(f32, GValue::Float);
impl_from_galue!(f64, GValue::Double);
impl_from_galue!(i32, GValue::Int32);
impl_from_galue!(i64, GValue::Int64);
impl_from_galue!(uuid::Uuid, GValue::Uuid);
impl_from_galue!(chrono::DateTime<chrono::Utc>, GValue::Date);
