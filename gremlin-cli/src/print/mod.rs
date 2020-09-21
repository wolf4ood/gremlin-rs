use gremlin_client::GValue;
mod edge;
mod gid;
mod gkey;
mod glist;
mod map;
mod path;
mod vertex;
use anyhow::{anyhow, Result};

pub fn fmt(value: &GValue) -> Result<String> {
    let result = match value {
        GValue::Vertex(v) => vertex::fmt(v),
        GValue::Edge(e) => edge::fmt(e),
        GValue::Map(map) =>  map::fmt(map)?,
        GValue::List(list) => glist::fmt(list)?,
        GValue::Int32(n) => n.to_string(),
        GValue::Int64(n) => n.to_string(),
        GValue::String(n) => n.to_string(),
        GValue::Uuid(uuid) => uuid.to_string(),
        GValue::Date(date) => date.to_string(),
        GValue::Double(n) => n.to_string(),
        GValue::Path(path) => path::fmt(path)?,
        _ => return Err(anyhow!("Printing no supported for {:?}", value)),
    };

    Ok(result)
}
