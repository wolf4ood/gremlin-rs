use crate::structure::{GValue, Vertex};


pub enum Either3<A: Into<GValue>, B: Into<GValue>, C: Into<GValue>> {
    A(A),
    B(B),
    C(C),
}

impl<A, B, C> From<Either3<A, B, C>> for GValue
where
    A: Into<GValue>,
    B: Into<GValue>,
    C: Into<GValue>,
{
    fn from(val: Either3<A, B, C>) -> Self {
        match val {
            Either3::A(a) => a.into(),
            Either3::B(b) => b.into(),
            Either3::C(c) => c.into(),
        }
    }
}

impl From<&str> for Either3<String, Vertex, GValue> {
    fn from(val: &str) -> Self {
        Either3::A(String::from(val))
    }
}

impl From<&Vertex> for Either3<String, Vertex, GValue> {
    fn from(val: &Vertex) -> Self {
        Either3::B(val.clone())
    }
}

impl From<GValue> for Either3<String, Vertex, GValue> {
    fn from(val: GValue) -> Self {
        Either3::C(val)
    }
}

impl From<&GValue> for Either3<String, Vertex, GValue> {
    fn from(val: &GValue) -> Self {
        Either3::C(val.clone())
    }
}
