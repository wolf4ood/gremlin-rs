use super::gid;
use super::vertex;
use gremlin_client::Edge;

pub fn fmt(edge: &Edge) -> String {
    format!(
        "e[{}][{}-{}->{}]",
        gid::fmt(edge.id()),
        vertex::fmt(edge.in_v()),
        edge.label(),
        vertex::fmt(edge.out_v())
    )
}
