use crate::conversion::ToGID;
use crate::conversion::ToGValue;
use crate::process::bytecode::Bytecode;
use crate::process::graph_traversal::GraphTraversal;
use crate::process::strategies::{RemoteStrategy, TraversalStrategy};
use crate::structure::{Edge, Vertex};
use crate::GremlinClient;
use std::sync::Arc;

#[derive(Clone)]
pub struct GraphTraversalSource {
    inner: Arc<InnerGraphTraversalSource>,
}

impl GraphTraversalSource {
    pub fn new(strategies: Vec<Box<TraversalStrategy>>) -> GraphTraversalSource {
        GraphTraversalSource {
            inner: Arc::new(InnerGraphTraversalSource {
                strategies: strategies.into_iter().map(Arc::from).collect(),
            }),
        }
    }

    pub fn with_remote(&self, client: GremlinClient) -> GraphTraversalSource {
        let mut strategies = self.inner.strategies.clone();

        strategies.push(Arc::new(RemoteStrategy::new(client)));

        GraphTraversalSource {
            inner: Arc::new(InnerGraphTraversalSource { strategies }),
        }
    }

    pub fn v(&self, ids: &[&dyn ToGID]) -> GraphTraversal<Vertex, Vertex> {
        let strategies = self.inner.strategies.clone();
        let mut code = Bytecode::new();

        code.add_step(
            String::from("V"),
            ids.iter().map(|id| id.to_gid().to_gvalue()).collect(),
        );

        GraphTraversal::new(strategies, code)
    }

    pub fn e(&self, ids: &[&dyn ToGID]) -> GraphTraversal<Edge, Edge> {
        let strategies = self.inner.strategies.clone();
        let mut code = Bytecode::new();

        code.add_step(
            String::from("E"),
            ids.iter().map(|id| id.to_gid().to_gvalue()).collect(),
        );

        GraphTraversal::new(strategies, code)
    }
}
pub struct InnerGraphTraversalSource {
    strategies: Vec<Arc<TraversalStrategy>>,
}

// TESTS
#[cfg(test)]
mod tests {

    use super::GraphTraversalSource;
    use crate::process::bytecode::Bytecode;
    use crate::process::P;
    use crate::GValue;

    #[test]
    fn v_traversal() {
        let g = GraphTraversalSource::new(vec![]);

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![1.into()]);

        assert_eq!(&code, g.v(&[&1]).bytecode());
    }

    #[test]
    fn e_traversal() {
        let g = GraphTraversalSource::new(vec![]);

        let mut code = Bytecode::new();

        code.add_step(String::from("E"), vec![1.into()]);

        assert_eq!(&code, g.e(&[&1]).bytecode());
    }
    #[test]
    fn v_has_label_traversal() {
        let g = GraphTraversalSource::new(vec![]);

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![1.into()]);
        code.add_step(
            String::from("hasLabel"),
            vec![String::from("person").into()],
        );

        assert_eq!(&code, g.v(&[&1]).has_label(&["person"]).bytecode());
    }

    #[test]
    fn v_has_traversal() {
        let g = GraphTraversalSource::new(vec![]);

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![1.into()]);
        code.add_step(
            String::from("has"),
            vec![
                String::from("name").into(),
                P::new("eq", String::from("marko").into()).into(),
            ],
        );
        code.add_step(
            String::from("has"),
            vec![String::from("age").into(), P::new("eq", 23.into()).into()],
        );

        assert_eq!(
            &code,
            g.v(&[&1]).has("name", &"marko").has("age", &23).bytecode()
        );
    }
}
