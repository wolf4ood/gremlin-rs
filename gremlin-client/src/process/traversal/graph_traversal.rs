use crate::conversion::FromGValue;
use crate::process::traversal::step::by::IntoByStep;
use crate::process::traversal::step::dedup::DedupStep;
use crate::process::traversal::step::has::IntoHasStep;
use crate::process::traversal::step::limit::LimitStep;
use crate::process::traversal::step::match_step::IntoMatchStep;
use crate::process::traversal::step::not::IntoNotStep;
use crate::process::traversal::step::or::IntoOrStep;
use crate::process::traversal::step::select::IntoSelectStep;
use crate::process::traversal::step::where_step::IntoWhereStep;

use crate::process::traversal::remote::Terminator;
use crate::process::traversal::{Bytecode, Scope, TraversalBuilder};
use crate::structure::Either3;
use crate::structure::Labels;
use crate::{
    structure::GProperty, structure::IntoPredicate, Edge, GValue, List, Map, Path, Vertex,
};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct GraphTraversal<S, E: FromGValue, T: Terminator<E>> {
    start: PhantomData<S>,
    end: PhantomData<E>,
    pub(crate) builder: TraversalBuilder,
    terminator: T,
}

impl<S, E: FromGValue, T: Terminator<E>> GraphTraversal<S, E, T> {
    pub fn new(terminator: T, builder: TraversalBuilder) -> GraphTraversal<S, E, T> {
        GraphTraversal {
            start: PhantomData,
            end: PhantomData,
            builder,
            terminator,
        }
    }
    pub fn bytecode(&self) -> &Bytecode {
        &self.builder.bytecode
    }

    pub fn has_label<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.builder = self.builder.has_label(labels);
        self
    }

    pub fn add_v<A>(mut self, label: A) -> GraphTraversal<Vertex, Vertex, T>
    where
        A: Into<Labels>,
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.add_v(label);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn property<A>(mut self, key: &str, value: A) -> Self
    where
        A: Into<GValue>,
    {
        self.builder = self.builder.property(key, value);
        self
    }

    pub fn has<A>(mut self, step: A) -> Self
    where
        A: IntoHasStep,
    {
        self.builder = self.builder.has(step);

        self
    }

    pub fn has_not<A>(mut self, key: A) -> Self
    where
        A: Into<String>,
    {
        self.builder = self.builder.has_not(key);
        self
    }
    pub fn as_<A>(mut self, alias: A) -> Self
    where
        A: Into<String>,
    {
        self.builder = self.builder.as_(alias);

        self
    }

    pub fn add_e<A>(mut self, label: A) -> GraphTraversal<S, Edge, T>
    where
        A: Into<String>,
        T: Terminator<Edge>,
    {
        self.builder = self.builder.add_e(label);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn out<A>(mut self, labels: A) -> GraphTraversal<S, Vertex, T>
    where
        A: Into<Labels>,
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.out(labels);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn out_e<A>(mut self, labels: A) -> GraphTraversal<S, Edge, T>
    where
        A: Into<Labels>,
        T: Terminator<Edge>,
    {
        self.builder = self.builder.out_e(labels);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn out_v(mut self) -> GraphTraversal<S, Vertex, T>
    where
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.out_v();

        GraphTraversal::new(self.terminator, self.builder)
    }
    pub fn in_<A>(mut self, labels: A) -> GraphTraversal<S, Vertex, T>
    where
        A: Into<Labels>,
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.in_(labels);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn in_e<A>(mut self, labels: A) -> GraphTraversal<S, Edge, T>
    where
        A: Into<Labels>,
        T: Terminator<Edge>,
    {
        self.builder = self.builder.in_e(labels);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn in_v(mut self) -> GraphTraversal<S, Vertex, T>
    where
        T: Terminator<Vertex>,
    {
        self.builder = self.builder.in_v();

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn label(mut self) -> GraphTraversal<S, String, T>
    where
        T: Terminator<String>,
    {
        self.builder = self.builder.label();

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn to_list(&self) -> T::List {
        self.terminator.to_list(self)
    }

    pub fn next(&self) -> T::Next {
        self.terminator.next(self)
    }
    pub fn has_next(&self) -> T::HasNext {
        self.terminator.has_next(self)
    }

    pub fn iter(&self) -> T::Iter {
        self.terminator.iter(self)
    }

    pub fn from<A>(mut self, target: A) -> Self
    where
        A: Into<Either3<String, Vertex, GValue>>,
    {
        self.builder = self.builder.from(target);
        self
    }

    pub fn to<A>(mut self, target: A) -> Self
    where
        A: Into<Either3<String, Vertex, GValue>>,
    {
        self.builder = self.builder.to(target);

        self
    }

    pub fn properties<L>(mut self, labels: L) -> GraphTraversal<S, GProperty, T>
    where
        L: Into<Labels>,
        T: Terminator<GProperty>,
    {
        self.builder = self.builder.properties(labels);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn property_map<L>(mut self, labels: L) -> GraphTraversal<S, Map, T>
    where
        L: Into<Labels>,
        T: Terminator<Map>,
    {
        self.builder = self.builder.property_map(labels);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn values<L>(mut self, labels: L) -> GraphTraversal<S, GValue, T>
    where
        L: Into<Labels>,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.values(labels);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn value_map<L>(mut self, labels: L) -> GraphTraversal<S, Map, T>
    where
        L: Into<Labels>,
        T: Terminator<Map>,
    {
        self.builder = self.builder.value_map(labels);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn count(mut self) -> GraphTraversal<S, i64, T>
    where
        T: Terminator<i64>,
    {
        self.builder = self.builder.count();
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn group_count(mut self) -> GraphTraversal<S, Map, T>
    where
        T: Terminator<Map>,
    {
        self.builder = self.builder.group_count(None);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn group_count_as<A>(mut self, key: A) -> GraphTraversal<S, E, T>
    where
        T: Terminator<Map>,
        A: Into<String>,
    {
        self.builder = self.builder.group_count(Some(key.into()));
        self
    }

    pub fn group(mut self) -> GraphTraversal<S, Map, T>
    where
        T: Terminator<Map>,
    {
        self.builder = self.builder.group(None);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn group_as<A>(mut self, key: A) -> GraphTraversal<S, E, T>
    where
        T: Terminator<Map>,
        A: Into<String>,
    {
        self.builder = self.builder.group(Some(key.into()));
        self
    }

    pub fn by<A>(mut self, step: A) -> Self
    where
        A: IntoByStep,
    {
        self.builder = self.builder.by(step);
        self
    }

    pub fn select<A>(mut self, step: A) -> GraphTraversal<S, GValue, T>
    where
        A: IntoSelectStep,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.select(step);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn fold(mut self) -> GraphTraversal<S, List, T>
    where
        T: Terminator<List>,
    {
        self.builder = self.builder.fold();
        GraphTraversal::new(self.terminator, self.builder)
    }
    pub fn unfold(mut self) -> Self {
        self.builder = self.builder.unfold();
        self
    }

    pub fn path(mut self) -> GraphTraversal<S, Path, T>
    where
        T: Terminator<Path>,
    {
        self.builder = self.builder.path();
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn limit<A>(mut self, limit: A) -> Self
    where
        A: Into<LimitStep>,
    {
        self.builder = self.builder.limit(limit);

        self
    }

    pub fn dedup<A>(mut self, dedup: A) -> Self
    where
        A: Into<DedupStep>,
    {
        self.builder = self.builder.dedup(dedup);
        self
    }

    pub fn sum<A>(mut self, scope: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<Scope>,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.sum(scope);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn max<A>(mut self, scope: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<Scope>,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.max(scope);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn mean<A>(mut self, scope: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<Scope>,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.mean(scope);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn min<A>(mut self, scope: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<Scope>,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.min(scope);

        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn is<A>(mut self, val: A) -> Self
    where
        A: IntoPredicate,
    {
        self.builder = self.builder.is(val);

        self
    }

    pub fn where_<A>(mut self, step: A) -> Self
    where
        A: IntoWhereStep,
    {
        self.builder = self.builder.where_(step);

        self
    }

    pub fn not<A>(mut self, step: A) -> Self
    where
        A: IntoNotStep,
    {
        self.builder = self.builder.not(step);
        self
    }

    pub fn order<A>(mut self, scope: A) -> Self
    where
        A: Into<Scope>,
    {
        self.builder = self.builder.order(scope);

        self
    }

    pub fn match_<A>(mut self, step: A) -> GraphTraversal<S, Map, T>
    where
        A: IntoMatchStep,
        T: Terminator<Map>,
    {
        self.builder = self.builder.match_(step);
        GraphTraversal::new(self.terminator, self.builder)
    }

    pub fn drop(mut self) -> Self {
        self.builder = self.builder.drop();
        self
    }

    pub fn or<A>(mut self, step: A) -> Self
    where
        A: IntoOrStep,
    {
        self.builder = self.builder.or(step);
        self
    }

    pub fn project<A>(mut self, step: A) -> GraphTraversal<S, GValue, T>
    where
        A: IntoSelectStep,
        T: Terminator<GValue>,
    {
        self.builder = self.builder.project(step);
        GraphTraversal::new(self.terminator, self.builder)
    }
}
