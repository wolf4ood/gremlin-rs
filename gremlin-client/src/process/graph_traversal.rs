use crate::conversion::FromGValue;
use crate::process::bytecode::Bytecode;
use crate::process::strategies::TraversalStrategies;
use crate::structure::Either2;
use crate::structure::Labels;
use crate::structure::P as Predicate;
use crate::{Edge, GValue, GremlinResult, Vertex};
use std::marker::PhantomData;

pub struct GraphTraversal<S, E: FromGValue> {
    start: PhantomData<S>,
    end: PhantomData<E>,
    strategies: TraversalStrategies,
    bytecode: Bytecode,
}

impl<S, E: FromGValue> GraphTraversal<S, E> {
    pub fn new(strategies: TraversalStrategies, bytecode: Bytecode) -> GraphTraversal<S, E> {
        GraphTraversal {
            start: PhantomData,
            end: PhantomData,
            bytecode,
            strategies,
        }
    }
    pub fn bytecode(&self) -> &Bytecode {
        &self.bytecode
    }

    pub fn has_label<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("hasLabel"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        self
    }

    pub fn add_v<T>(mut self, label: T) -> GraphTraversal<Vertex, Vertex>
    where
        T: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("addV"),
            label.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn property<T>(mut self, key: &str, value: T) -> Self
    where
        T: Into<GValue>,
    {
        self.bytecode.add_step(
            String::from("property"),
            vec![String::from(key).into(), value.into()],
        );
        self
    }
    pub fn has<P>(mut self, key: &str, predicate: P) -> Self
    where
        P: Into<Predicate>,
    {
        let p = predicate.into();
        self.bytecode.add_step(
            String::from("has"),
            vec![String::from(key).into(), p.into()],
        );
        self
    }

    pub fn as_<T>(mut self, alias: T) -> GraphTraversal<S, E>
    where
        T: Into<String>,
    {
        self.bytecode
            .add_step(String::from("as"), vec![alias.into().into()]);

        self
    }

    pub fn add_e<T>(mut self, label: T) -> GraphTraversal<S, Edge>
    where
        T: Into<String>,
    {
        self.bytecode
            .add_step(String::from("addE"), vec![label.into().into()]);

        GraphTraversal::new(self.strategies, self.bytecode)
    }
    pub fn out<L>(mut self, labels: L) -> GraphTraversal<S, Vertex>
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("out"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn out_e<L>(mut self, labels: L) -> GraphTraversal<S, Edge>
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("outE"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn in_<L>(mut self, labels: L) -> GraphTraversal<S, Vertex>
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("in"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn in_e<L>(mut self, labels: L) -> GraphTraversal<S, Edge>
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("inE"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.strategies, self.bytecode)
    }

    pub fn to_list(&self) -> GremlinResult<Vec<E>> {
        self.strategies.apply(self)?.collect()
    }

    pub fn from<A>(mut self, target: A) -> GraphTraversal<S, E>
    where
        A: Into<Either2<String, Vertex>>,
    {
        self.bytecode
            .add_step(String::from("from"), vec![target.into().into()]);

        self
    }

    pub fn to<A>(mut self, target: A) -> GraphTraversal<S, E>
    where
        A: Into<Either2<String, Vertex>>,
    {
        self.bytecode
            .add_step(String::from("to"), vec![target.into().into()]);

        self
    }
}
