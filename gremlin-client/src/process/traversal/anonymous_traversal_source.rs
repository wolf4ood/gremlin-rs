use crate::process::traversal::remote::MockTerminator;
use crate::process::traversal::step::has::IntoHasStep;
use crate::process::traversal::step::not::IntoNotStep;
use crate::process::traversal::Bytecode;
use crate::process::traversal::GraphTraversal;

use crate::structure::{GValue, Labels, Vertex};

pub struct AnonymousTraversalSource {
    traversal: GraphTraversal<GValue, GValue, MockTerminator>,
}

impl AnonymousTraversalSource {
    pub fn new() -> AnonymousTraversalSource {
        AnonymousTraversalSource {
            traversal: GraphTraversal::new(MockTerminator::new(), Bytecode::default()),
        }
    }

    pub fn count(&self) -> GraphTraversal<GValue, i64, MockTerminator> {
        self.traversal.clone().count()
    }

    pub fn out<L>(&self, labels: L) -> GraphTraversal<GValue, Vertex, MockTerminator>
    where
        L: Into<Labels>,
    {
        self.traversal.clone().out(labels)
    }

    pub fn values<L>(&self, labels: L) -> GraphTraversal<GValue, GValue, MockTerminator>
    where
        L: Into<Labels>,
    {
        self.traversal.clone().values(labels)
    }
    pub fn has_label<L>(&self, labels: L) -> GraphTraversal<GValue, GValue, MockTerminator>
    where
        L: Into<Labels>,
    {
        self.traversal.clone().has_label(labels)
    }

    pub fn as_<A>(&self, alias: A) -> GraphTraversal<GValue, GValue, MockTerminator>
    where
        A: Into<String>,
    {
        self.traversal.clone().as_(alias)
    }
    pub fn has<A>(&self, step: A) -> GraphTraversal<GValue, GValue, MockTerminator>
    where
        A: IntoHasStep,
    {
        self.traversal.clone().has(step)
    }

    pub fn not<A>(&self, step: A) -> GraphTraversal<GValue, GValue, MockTerminator>
    where
        A: IntoNotStep,
    {
        self.traversal.clone().not(step)
    }
}

impl Default for AnonymousTraversalSource {
    fn default() -> Self {
        Self::new()
    }
}
