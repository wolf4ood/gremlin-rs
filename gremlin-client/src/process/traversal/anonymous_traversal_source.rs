use crate::process::traversal::step::has::IntoHasStep;
use crate::process::traversal::strategies::TraversalStrategies;
use crate::process::traversal::Bytecode;
use crate::process::traversal::GraphTraversal;

use crate::structure::{GValue, Labels, Vertex};

pub struct AnonymousTraversalSource {
    traversal: GraphTraversal<GValue, GValue>,
}

impl AnonymousTraversalSource {
    pub fn new() -> AnonymousTraversalSource {
        AnonymousTraversalSource {
            traversal: GraphTraversal::new(TraversalStrategies::new(vec![]), Bytecode::default()),
        }
    }

    pub fn count(&self) -> GraphTraversal<GValue, i64> {
        self.traversal.clone().count()
    }

    pub fn out<L>(&self, labels: L) -> GraphTraversal<GValue, Vertex>
    where
        L: Into<Labels>,
    {
        self.traversal.clone().out(labels)
    }

    pub fn values<L>(&self, labels: L) -> GraphTraversal<GValue, GValue>
    where
        L: Into<Labels>,
    {
        self.traversal.clone().values(labels)
    }
    pub fn has_label<L>(&self, labels: L) -> GraphTraversal<GValue, GValue>
    where
        L: Into<Labels>,
    {
        self.traversal.clone().has_label(labels)
    }

    pub fn has<A>(&self, step: A) -> GraphTraversal<GValue, GValue>
    where
        A: IntoHasStep,
    {
        self.traversal.clone().has(step)
    }
}

impl Default for AnonymousTraversalSource {
    fn default() -> Self {
        Self::new()
    }
}
