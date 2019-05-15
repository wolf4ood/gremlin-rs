use crate::conversion::ToGValue;
use crate::process::traversal::strategies::{
    RemoteStrategy, TraversalStrategies, TraversalStrategy,
};
use crate::process::traversal::Bytecode;
use crate::process::traversal::GraphTraversal;
use crate::structure::GIDs;
use crate::structure::Labels;
use crate::structure::{Edge, GValue, Vertex};
use crate::GremlinClient;
use std::sync::Arc;

#[derive(Clone)]
pub struct GraphTraversalSource {
    inner: Arc<InnerGraphTraversalSource>,
}

impl GraphTraversalSource {
    pub fn new(strategies: TraversalStrategies) -> GraphTraversalSource {
        GraphTraversalSource {
            inner: Arc::new(InnerGraphTraversalSource { strategies }),
        }
    }

    pub fn empty() -> GraphTraversalSource {
        GraphTraversalSource::new(TraversalStrategies::new(vec![]))
    }

    pub fn with_remote(&self, client: GremlinClient) -> GraphTraversalSource {
        let mut strategies = self.inner.strategies.clone();

        strategies.add_strategy(TraversalStrategy::Remote(RemoteStrategy::new(client)));

        GraphTraversalSource {
            inner: Arc::new(InnerGraphTraversalSource { strategies }),
        }
    }

    pub fn v<T>(&self, ids: T) -> GraphTraversal<Vertex, Vertex>
    where
        T: Into<GIDs>,
    {
        let strategies = self.inner.strategies.clone();
        let mut code = Bytecode::new();

        code.add_step(
            String::from("V"),
            ids.into().0.iter().map(|id| id.to_gvalue()).collect(),
        );

        GraphTraversal::new(strategies, code)
    }

    pub fn add_v<T>(&self, label: T) -> GraphTraversal<Vertex, Vertex>
    where
        T: Into<Labels>,
    {
        let strategies = self.inner.strategies.clone();
        let mut code = Bytecode::new();

        code.add_step(
            String::from("addV"),
            label.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(strategies, code)
    }

    pub fn add_e<T>(&self, label: T) -> GraphTraversal<Edge, Edge>
    where
        T: Into<String>,
    {
        let strategies = self.inner.strategies.clone();
        let mut code = Bytecode::new();

        code.add_step(String::from("addE"), vec![label.into().into()]);

        GraphTraversal::new(strategies, code)
    }

    pub fn e<T>(&self, ids: T) -> GraphTraversal<Edge, Edge>
    where
        T: Into<GIDs>,
    {
        let strategies = self.inner.strategies.clone();
        let mut code = Bytecode::new();

        code.add_step(
            String::from("E"),
            ids.into().0.iter().map(|id| id.to_gvalue()).collect(),
        );

        GraphTraversal::new(strategies, code)
    }
}
pub struct InnerGraphTraversalSource {
    strategies: TraversalStrategies,
}

// TESTS
#[cfg(test)]
mod tests {

    use super::GraphTraversalSource;
    use crate::process::traversal::strategies::TraversalStrategies;
    use crate::process::traversal::{Bytecode, Scope, __};
    use crate::structure::{GValue, P, T};

    #[test]
    fn v_traversal() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![1.into()]);

        assert_eq!(&code, g.v(1).bytecode());
    }

    #[test]
    fn e_traversal() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("E"), vec![1.into()]);

        assert_eq!(&code, g.e(1).bytecode());
    }
    #[test]
    fn v_has_label_traversal() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![1.into()]);
        code.add_step(
            String::from("hasLabel"),
            vec![String::from("person").into()],
        );

        assert_eq!(&code, g.v(1).has_label("person").bytecode());
    }

    #[test]
    fn v_has_traversal() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

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
            g.v(1).has(("name", "marko")).has(("age", 23)).bytecode()
        );

        // has with 3 params

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(
            String::from("has"),
            vec![
                String::from("person").into(),
                String::from("name").into(),
                P::new("eq", String::from("marko").into()).into(),
            ],
        );

        assert_eq!(&code, g.v(()).has(("person", "name", "marko")).bytecode());

        // has with 1 param

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("has"), vec![String::from("name").into()]);

        assert_eq!(&code, g.v(()).has("name").bytecode());

        // hasNot

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("hasNot"), vec![String::from("name").into()]);

        assert_eq!(&code, g.v(()).has_not("name").bytecode());
    }

    #[test]
    fn v_has_traversal_with_p() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        // EQ
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![1.into()]);

        code.add_step(
            String::from("has"),
            vec![String::from("age").into(), P::eq(23).into()],
        );
        assert_eq!(&code, g.v(1).has(("age", P::eq(23))).bytecode());

        // NEQ
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![1.into()]);

        code.add_step(
            String::from("has"),
            vec![String::from("age").into(), P::neq(23).into()],
        );
        assert_eq!(&code, g.v(1).has(("age", P::neq(23))).bytecode());

        // GTE
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![1.into()]);

        code.add_step(
            String::from("has"),
            vec![String::from("age").into(), P::gte(23).into()],
        );

        assert_eq!(&code, g.v(1).has(("age", P::gte(23))).bytecode());

        // GT
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![1.into()]);

        code.add_step(
            String::from("has"),
            vec![String::from("age").into(), P::gt(23).into()],
        );

        assert_eq!(&code, g.v(1).has(("age", P::gt(23))).bytecode());

        // LTE
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![1.into()]);

        code.add_step(
            String::from("has"),
            vec![String::from("age").into(), P::lte(23).into()],
        );
        assert_eq!(&code, g.v(1).has(("age", P::lte(23))).bytecode());

        // LT
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![1.into()]);

        code.add_step(
            String::from("has"),
            vec![String::from("age").into(), P::lt(23).into()],
        );
        assert_eq!(&code, g.v(1).has(("age", P::lt(23))).bytecode());
    }
    #[test]
    fn add_v_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("addV"), vec![String::from("person").into()]);

        assert_eq!(&code, g.add_v("person").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("addV"), vec![]);

        assert_eq!(&code, g.add_v(()).bytecode());
    }

    #[test]
    fn add_v_with_property_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("addV"), vec![String::from("person").into()]);
        code.add_step(
            String::from("property"),
            vec![String::from("name").into(), String::from("marko").into()],
        );

        assert_eq!(
            &code,
            g.add_v("person").property("name", "marko").bytecode()
        );
    }

    #[test]
    fn add_e_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("addE"), vec![String::from("knows").into()]);

        assert_eq!(&code, g.add_e("knows").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("addE"), vec![String::from("knows").into()]);
        code.add_step(String::from("from"), vec![String::from("a").into()]);
        code.add_step(String::from("to"), vec![String::from("b").into()]);

        assert_eq!(&code, g.add_e("knows").from("a").to("b").bytecode());
    }

    #[test]
    fn as_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("as"), vec![String::from("a").into()]);

        assert_eq!(&code, g.v(()).as_("a").bytecode());
    }

    #[test]
    fn label_step_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("label"), vec![]);

        assert_eq!(&code, g.v(()).label().bytecode());
    }

    #[test]
    fn properties_step_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("properties"), vec![]);

        assert_eq!(&code, g.v(()).properties(()).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(
            String::from("properties"),
            vec![String::from("name").into()],
        );

        assert_eq!(&code, g.v(()).properties("name").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(
            String::from("properties"),
            vec![String::from("name").into(), String::from("surname").into()],
        );

        assert_eq!(
            &code,
            g.v(()).properties(vec!["name", "surname"]).bytecode()
        );
    }

    #[test]
    fn property_map_step_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("propertyMap"), vec![]);

        assert_eq!(&code, g.v(()).property_map(()).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(
            String::from("propertyMap"),
            vec![String::from("name").into()],
        );

        assert_eq!(&code, g.v(()).property_map("name").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(
            String::from("propertyMap"),
            vec![String::from("name").into(), String::from("surname").into()],
        );

        assert_eq!(
            &code,
            g.v(()).property_map(vec!["name", "surname"]).bytecode()
        );
    }

    #[test]
    fn values_step_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("values"), vec![]);

        assert_eq!(&code, g.v(()).values(()).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("values"), vec![String::from("name").into()]);

        assert_eq!(&code, g.v(()).values("name").bytecode());
    }

    #[test]
    fn count_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("count"), vec![]);

        assert_eq!(&code, g.v(()).count().bytecode());
    }

    #[test]
    fn group_count_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("groupCount"), vec![]);

        assert_eq!(&code, g.v(()).group_count().bytecode());
    }

    #[test]
    fn group_count_by_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("groupCount"), vec![]);
        code.add_step(String::from("by"), vec![]);

        assert_eq!(&code, g.v(()).group_count().by(()).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("groupCount"), vec![]);
        code.add_step(String::from("by"), vec!["name".into()]);

        assert_eq!(&code, g.v(()).group_count().by("name").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("groupCount"), vec![]);
        code.add_step(String::from("by"), vec![T::Label.into()]);

        assert_eq!(&code, g.v(()).group_count().by(T::Label).bytecode());
    }

    #[test]
    fn group_by_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("group"), vec![]);
        code.add_step(String::from("by"), vec![]);

        assert_eq!(&code, g.v(()).group().by(()).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("group"), vec![]);
        code.add_step(String::from("by"), vec!["name".into()]);

        assert_eq!(&code, g.v(()).group().by("name").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("group"), vec![]);
        code.add_step(String::from("by"), vec![T::Label.into()]);

        assert_eq!(&code, g.v(()).group().by(T::Label).bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("group"), vec![]);
        code.add_step(String::from("by"), vec![T::Label.into()]);
        code.add_step(
            String::from("by"),
            vec![__.count().bytecode().clone().into()],
        );

        assert_eq!(
            &code,
            g.v(()).group().by(T::Label).by(__.count()).bytecode()
        );
    }

    #[test]
    fn select_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("select"), vec!["name".into()]);

        assert_eq!(&code, g.v(()).select("name").bytecode());

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(
            String::from("select"),
            vec!["name".into(), "surname".into()],
        );

        assert_eq!(&code, g.v(()).select(vec!["name", "surname"]).bytecode());
    }

    #[test]
    fn fold_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("values"), vec!["name".into()]);
        code.add_step(String::from("fold"), vec![]);

        assert_eq!(&code, g.v(()).values("name").fold().bytecode());
    }

    #[test]
    fn unfold_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("propertyMap"), vec![]);
        code.add_step(String::from("unfold"), vec![]);

        assert_eq!(&code, g.v(()).property_map(()).unfold().bytecode());
    }

    #[test]
    fn path_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("path"), vec![]);

        assert_eq!(&code, g.v(()).path().bytecode());
    }

    #[test]
    fn limit_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("limit"), vec![GValue::Int64(1)]);

        assert_eq!(&code, g.v(()).limit(1).bytecode());
    }

    #[test]
    fn dedup_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("values"), vec![]);
        code.add_step(String::from("dedup"), vec![]);

        assert_eq!(&code, g.v(()).values(()).dedup(()).bytecode());
    }

    #[test]
    fn numerical_test() {
        let g = GraphTraversalSource::new(TraversalStrategies::new(vec![]));

        // sum
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("values"), vec!["test".into()]);
        code.add_step(String::from("sum"), vec![Scope::Global.into()]);

        assert_eq!(&code, g.v(()).values("test").sum(()).bytecode());

        // max
        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("values"), vec!["test".into()]);
        code.add_step(String::from("max"), vec![Scope::Global.into()]);

        assert_eq!(&code, g.v(()).values("test").max(()).bytecode());

        // mean

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("values"), vec!["test".into()]);
        code.add_step(String::from("mean"), vec![Scope::Global.into()]);

        assert_eq!(&code, g.v(()).values("test").mean(()).bytecode());

        // min

        let mut code = Bytecode::new();

        code.add_step(String::from("V"), vec![]);
        code.add_step(String::from("values"), vec!["test".into()]);
        code.add_step(String::from("min"), vec![Scope::Global.into()]);

        assert_eq!(&code, g.v(()).values("test").min(()).bytecode());
    }

}
