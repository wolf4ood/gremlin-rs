use crate::process::traversal::strategies::TraversalStrategies;
use crate::process::traversal::Bytecode;
use crate::process::traversal::GraphTraversal;

use crate::structure::GValue;

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
}
