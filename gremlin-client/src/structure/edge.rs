use crate::structure::{Property, Vertex, GID};
use std::collections::HashMap;
use std::hash::Hasher;

#[derive(Debug, Clone)]
pub struct Edge {
    id: GID,
    label: String,
    in_v: Vertex,
    out_v: Vertex,
    properties: HashMap<String, Property>,
}

impl Edge {
    pub(crate) fn new<T>(
        id: GID,
        label: T,
        in_v_id: GID,
        in_v_label: T,
        out_v_id: GID,
        out_v_label: T,
        properties: HashMap<String, Property>,
    ) -> Edge
    where
        T: Into<String>,
    {
        Edge {
            id,
            label: label.into(),
            in_v: Vertex::new(in_v_id, in_v_label, HashMap::new()),
            out_v: Vertex::new(out_v_id, out_v_label, HashMap::new()),
            properties,
        }
    }

    pub fn id(&self) -> &GID {
        &self.id
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn in_v(&self) -> &Vertex {
        &self.in_v
    }
    pub fn out_v(&self) -> &Vertex {
        &self.out_v
    }
}

impl std::cmp::Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        &self.id == other.id()
    }
}

impl std::hash::Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
