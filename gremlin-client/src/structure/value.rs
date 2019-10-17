use crate::conversion::{BorrowFromGValue, FromGValue};
use crate::process::traversal::{Bytecode, Order, Scope};
use crate::structure::traverser::Traverser;
use crate::structure::{
    label::LabelType, Edge, GKey, IntermediateRepr, List, Map, Metric, Path, Property, Set, Token,
    TraversalExplanation, TraversalMetrics, Vertex, VertexProperty,
};
use crate::structure::{P, T};
use crate::GremlinResult;
use chrono;
use std::collections::{BTreeMap, HashMap, VecDeque};
pub type Date = chrono::DateTime<chrono::offset::Utc>;

/// Represent possible values coming from the [Gremlin Server](http://tinkerpop.apache.org/docs/3.4.0/dev/io/)
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Clone)]
pub enum GValue {
    Vertex(Vertex),
    Edge(Edge),
    VertexProperty(VertexProperty),
    Property(Property),
    Uuid(uuid::Uuid),
    Int32(i32),
    Int64(i64),
    Float(f32),
    Double(f64),
    Date(Date),
    List(List),
    Set(Set),
    Map(Map),
    Token(Token),
    String(String),
    Path(Path),
    TraversalMetrics(TraversalMetrics),
    Metric(Metric),
    TraversalExplanation(TraversalExplanation),
    IntermediateRepr(IntermediateRepr),
    P(P),
    T(T),
    Bytecode(Bytecode),
    Traverser(Traverser),
    Scope(Scope),
    Order(Order),
    Bool(bool),
}

impl GValue {
    pub fn take<T>(self) -> GremlinResult<T>
    where
        T: FromGValue,
    {
        T::from_gvalue(self)
    }

    pub fn get<'a, T>(&'a self) -> GremlinResult<&'a T>
    where
        T: BorrowFromGValue,
    {
        T::from_gvalue(self)
    }
}

impl From<Date> for GValue {
    fn from(val: Date) -> Self {
        GValue::Date(val)
    }
}

impl From<String> for GValue {
    fn from(val: String) -> Self {
        GValue::String(val)
    }
}

impl From<i32> for GValue {
    fn from(val: i32) -> Self {
        GValue::Int32(val)
    }
}

impl From<i64> for GValue {
    fn from(val: i64) -> Self {
        GValue::Int64(val)
    }
}

impl From<f32> for GValue {
    fn from(val: f32) -> Self {
        GValue::Float(val)
    }
}
impl From<f64> for GValue {
    fn from(val: f64) -> Self {
        GValue::Double(val)
    }
}

impl<'a> From<&'a str> for GValue {
    fn from(val: &'a str) -> Self {
        GValue::String(String::from(val))
    }
}

impl From<Vertex> for GValue {
    fn from(val: Vertex) -> Self {
        GValue::Vertex(val)
    }
}

impl From<&Vertex> for GValue {
    fn from(val: &Vertex) -> Self {
        GValue::Vertex(val.clone())
    }
}

impl From<Path> for GValue {
    fn from(val: Path) -> Self {
        GValue::Path(val)
    }
}
impl From<Edge> for GValue {
    fn from(val: Edge) -> Self {
        GValue::Edge(val)
    }
}

impl From<VertexProperty> for GValue {
    fn from(val: VertexProperty) -> Self {
        GValue::VertexProperty(val)
    }
}

impl From<Traverser> for GValue {
    fn from(val: Traverser) -> Self {
        GValue::Traverser(val)
    }
}
impl From<TraversalMetrics> for GValue {
    fn from(val: TraversalMetrics) -> Self {
        GValue::TraversalMetrics(val)
    }
}

impl From<TraversalExplanation> for GValue {
    fn from(val: TraversalExplanation) -> Self {
        GValue::TraversalExplanation(val)
    }
}

impl From<Metric> for GValue {
    fn from(val: Metric) -> Self {
        GValue::Metric(val)
    }
}

impl From<Property> for GValue {
    fn from(val: Property) -> Self {
        GValue::Property(val)
    }
}

impl From<Scope> for GValue {
    fn from(val: Scope) -> Self {
        GValue::Scope(val)
    }
}

impl From<Order> for GValue {
    fn from(val: Order) -> Self {
        GValue::Order(val)
    }
}
impl From<Token> for GValue {
    fn from(val: Token) -> Self {
        GValue::Token(val)
    }
}

impl From<HashMap<String, GValue>> for GValue {
    fn from(val: HashMap<String, GValue>) -> Self {
        GValue::Map(Map::from(val))
    }
}

impl From<HashMap<GKey, GValue>> for GValue {
    fn from(val: HashMap<GKey, GValue>) -> Self {
        GValue::Map(Map::from(val))
    }
}

impl From<BTreeMap<String, GValue>> for GValue {
    fn from(val: BTreeMap<String, GValue>) -> Self {
        GValue::Map(Map::from(val))
    }
}

impl From<Vec<GValue>> for GValue {
    fn from(val: Vec<GValue>) -> Self {
        GValue::List(List::new(val))
    }
}

impl From<GValue> for Vec<GValue> {
    fn from(val: GValue) -> Self {
        vec![val]
    }
}

impl From<GValue> for VecDeque<GValue> {
    fn from(val: GValue) -> Self {
        match val {
            GValue::List(l) => VecDeque::from(l.take()),
            GValue::Set(l) => VecDeque::from(l.take()),
            _ => VecDeque::from(vec![val]),
        }
    }
}

impl From<GKey> for GValue {
    fn from(val: GKey) -> Self {
        match val {
            GKey::String(s) => GValue::String(s),
            GKey::Token(s) => GValue::String(s.value().clone()),
            GKey::Vertex(v) => GValue::Vertex(v),
            GKey::Edge(v) => GValue::Edge(v),
        }
    }
}

impl From<P> for GValue {
    fn from(val: P) -> GValue {
        GValue::P(val)
    }
}

impl From<T> for GValue {
    fn from(val: T) -> GValue {
        GValue::T(val)
    }
}

impl From<Bytecode> for GValue {
    fn from(val: Bytecode) -> GValue {
        GValue::Bytecode(val)
    }
}

impl From<bool> for GValue {
    fn from(val: bool) -> GValue {
        GValue::Bool(val)
    }
}

impl From<LabelType> for GValue {
    fn from(val: LabelType) -> GValue {
        match val {
            LabelType::String_(val) => val.into(),
            LabelType::Bool(val) => val.into(),
        }
    }
}
