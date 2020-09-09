use super::gid;
use gremlin_client::Vertex;

pub fn fmt(vertex: &Vertex) -> String {
    format!("v:{}[{}]", vertex.label(), gid::fmt(vertex.id()))
}
