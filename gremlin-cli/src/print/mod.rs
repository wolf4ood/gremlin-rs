use gremlin_client::GValue;
mod edge;
mod gid;
mod gkey;
mod glist;
mod map;
mod vertex;

pub fn fmt(value: &GValue) -> String {
    match value {
        GValue::Vertex(v) => format!("{}", vertex::fmt(v)),
        GValue::Edge(e) => format!("{}", edge::fmt(e)),
        GValue::Map(map) => format!("{}", map::fmt(map)),
        GValue::List(list) => format!("{}", glist::fmt(list)),
        GValue::Int32(n) => format!("{}", n),
        GValue::Int64(n) => format!("{}", n),
        GValue::String(n) => format!("{}", n),
        GValue::Uuid(uuid) => format!("{}", uuid),
        GValue::Date(date) => format!("{}", date),
        GValue::Double(n) => format!("{}", n),
        _ => panic!("Printing no supported for {:?}", value),
    }
}
