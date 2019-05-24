use crate::conversion::FromGValue;
use crate::process::traversal::step::by::IntoByStep;
use crate::process::traversal::step::dedup::DedupStep;
use crate::process::traversal::step::has::IntoHasStep;
use crate::process::traversal::step::limit::LimitStep;
use crate::process::traversal::step::match_step::IntoMatchStep;
use crate::process::traversal::step::not::IntoNotStep;
use crate::process::traversal::step::select::IntoSelectStep;
use crate::process::traversal::step::where_step::IntoWhereStep;

use crate::process::traversal::remote::Terminator;
use crate::process::traversal::{Bytecode, Scope};
use crate::structure::Either2;
use crate::structure::Labels;
use crate::{
    structure::GProperty, structure::IntoPredicate, Edge, GValue, List, Map, Path, Vertex,
};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct GraphTraversal<S, E: FromGValue, T: Terminator<E>> {
    start: PhantomData<S>,
    end: PhantomData<E>,
    pub(crate) bytecode: Bytecode,
    terminator: T,
}

impl<S, E: FromGValue, T: Terminator<E>> GraphTraversal<S, E, T> {
    pub fn new(terminator: T, bytecode: Bytecode) -> GraphTraversal<S, E, T> {
        GraphTraversal {
            start: PhantomData,
            end: PhantomData,
            bytecode,
            terminator,
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

    pub fn add_v<A>(mut self, label: A) -> GraphTraversal<Vertex, Vertex, T>
    where
        A: Into<Labels>,
        T: Terminator<Vertex>,
    {
        self.bytecode.add_step(
            String::from("addV"),
            label.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn property<A>(mut self, key: &str, value: A) -> Self
    where
        A: Into<GValue>,
    {
        self.bytecode.add_step(
            String::from("property"),
            vec![String::from(key).into(), value.into()],
        );
        self
    }

    pub fn has<A>(mut self, step: A) -> Self
    where
        A: IntoHasStep,
    {
        self.bytecode
            .add_step(String::from("has"), step.into_step().take_params());
        self
    }

    pub fn has_not<A>(mut self, key: A) -> Self
    where
        A: Into<String>,
    {
        self.bytecode
            .add_step(String::from("hasNot"), vec![key.into().into()]);
        self
    }
    pub fn as_<A>(mut self, alias: A) -> Self
    where
        A: Into<String>,
    {
        self.bytecode
            .add_step(String::from("as"), vec![alias.into().into()]);

        self
    }

    pub fn add_e<A>(mut self, label: A) -> GraphTraversal<S, Edge, T>
    where
        A: Into<String>,
        T: Terminator<Edge>,
    {
        self.bytecode
            .add_step(String::from("addE"), vec![label.into().into()]);

        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn out<A>(mut self, labels: A) -> GraphTraversal<S, Vertex, T>
    where
        A: Into<Labels>,
        T: Terminator<Vertex>,
    {
        self.bytecode.add_step(
            String::from("out"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn out_e<A>(mut self, labels: A) -> GraphTraversal<S, Edge, T>
    where
        A: Into<Labels>,
        T: Terminator<Edge>,
    {
        self.bytecode.add_step(
            String::from("outE"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn out_v(mut self) -> GraphTraversal<S, Vertex, T>
    where
        T: Terminator<Vertex>,
    {
        self.bytecode.add_step(String::from("outV"), vec![]);

        GraphTraversal::new(self.terminator, self.bytecode)
    }
    pub fn in_<A>(mut self, labels: A) -> GraphTraversal<S, Vertex, T>
    where
        A: Into<Labels>,
        T: Terminator<Vertex>,
    {
        self.bytecode.add_step(
            String::from("in"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn in_e<A>(mut self, labels: A) -> GraphTraversal<S, Edge, T>
    where
        A: Into<Labels>,
        T: Terminator<Edge>,
    {
        self.bytecode.add_step(
            String::from("inE"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn in_v(mut self) -> GraphTraversal<S, Vertex, T>
    where
        T: Terminator<Vertex>,
    {
        self.bytecode.add_step(String::from("inV"), vec![]);

        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn label(mut self) -> GraphTraversal<S, String, T>
    where
        T: Terminator<String>,
    {
        self.bytecode.add_step(String::from("label"), vec![]);

        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn to_list(&self) -> T::List {
        self.terminator.to_list(self)
    }

    pub fn from<A>(mut self, target: A) -> Self
    where
        A: Into<Either2<String, Vertex>>,
    {
        self.bytecode
            .add_step(String::from("from"), vec![target.into().into()]);

        self
    }

    pub fn to<A>(mut self, target: A) -> Self
    where
        A: Into<Either2<String, Vertex>>,
    {
        self.bytecode
            .add_step(String::from("to"), vec![target.into().into()]);

        self
    }

    pub fn properties<L>(mut self, labels: L) -> GraphTraversal<S, GProperty, T>
    where
        L: Into<Labels>,
        T: Terminator<GProperty>,
    {
        self.bytecode.add_step(
            String::from("properties"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn property_map<L>(mut self, labels: L) -> GraphTraversal<S, Map, T>
    where
        L: Into<Labels>,
        T: Terminator<Map>,
    {
        self.bytecode.add_step(
            String::from("propertyMap"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn values<L>(mut self, labels: L) -> GraphTraversal<S, GValue, T>
    where
        L: Into<Labels>,
        T: Terminator<GValue>,
    {
        self.bytecode.add_step(
            String::from("values"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn value_map<L>(mut self, labels: L) -> GraphTraversal<S, Map, T>
    where
        L: Into<Labels>,
        T: Terminator<Map>,
    {
        self.bytecode.add_step(
            String::from("valueMap"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn count(mut self) -> GraphTraversal<S, i64, T>
    where
        T: Terminator<i64>,
    {
        self.bytecode.add_step(String::from("count"), vec![]);
        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn group_count(mut self) -> GraphTraversal<S, Map, T>
    where
        T: Terminator<Map>,
    {
        self.bytecode.add_step(String::from("groupCount"), vec![]);
        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn group(mut self) -> GraphTraversal<S, Map, T>
    where
        T: Terminator<Map>,
    {
        self.bytecode.add_step(String::from("group"), vec![]);
        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn by<A>(mut self, step: A) -> Self
    where
        A: IntoByStep,
    {
        self.bytecode
            .add_step(String::from("by"), step.into_step().take_params());
        self
    }

    pub fn select<A>(mut self, step: A) -> GraphTraversal<S, GValue, T>
    where
        A: IntoSelectStep,
        T: Terminator<GValue>,
    {
        self.bytecode
            .add_step(String::from("select"), step.into_step().take_params());
        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn fold(mut self) -> GraphTraversal<S, List, T>
    where
        T: Terminator<List>,
    {
        self.bytecode.add_step(String::from("fold"), vec![]);
        GraphTraversal::new(self.terminator, self.bytecode)
    }
    pub fn unfold(mut self) -> Self {
        self.bytecode.add_step(String::from("unfold"), vec![]);
        self
    }

    pub fn path(mut self) -> GraphTraversal<S, Path, T>
    where
        T: Terminator<Path>,
    {
        self.bytecode.add_step(String::from("path"), vec![]);
        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn limit<A>(mut self, limit: A) -> Self
    where
        A: Into<LimitStep>,
    {
        self.bytecode
            .add_step(String::from("limit"), limit.into().params());

        self
    }

    pub fn dedup<A>(mut self, limit: A) -> Self
    where
        A: Into<DedupStep>,
    {
        self.bytecode
            .add_step(String::from("dedup"), limit.into().params());

        self
    }

    pub fn sum<A>(mut self, scope: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<Scope>,
        T: Terminator<GValue>,
    {
        self.bytecode
            .add_step(String::from("sum"), vec![scope.into().into()]);

        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn max<A>(mut self, scope: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<Scope>,
        T: Terminator<GValue>,
    {
        self.bytecode
            .add_step(String::from("max"), vec![scope.into().into()]);

        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn mean<A>(mut self, scope: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<Scope>,
        T: Terminator<GValue>,
    {
        self.bytecode
            .add_step(String::from("mean"), vec![scope.into().into()]);

        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn min<A>(mut self, scope: A) -> GraphTraversal<S, GValue, T>
    where
        A: Into<Scope>,
        T: Terminator<GValue>,
    {
        self.bytecode
            .add_step(String::from("min"), vec![scope.into().into()]);

        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn is<A>(mut self, val: A) -> Self
    where
        A: IntoPredicate,
    {
        self.bytecode
            .add_step(String::from("is"), vec![val.into_predicate().into()]);

        self
    }

    pub fn where_<A>(mut self, step: A) -> Self
    where
        A: IntoWhereStep,
    {
        self.bytecode
            .add_step(String::from("where"), step.into_step().take_params());
        self
    }

    pub fn not<A>(mut self, step: A) -> Self
    where
        A: IntoNotStep,
    {
        self.bytecode
            .add_step(String::from("not"), step.into_step().take_params());
        self
    }

    pub fn order<A>(mut self, scope: A) -> Self
    where
        A: Into<Scope>,
    {
        self.bytecode
            .add_step(String::from("order"), vec![scope.into().into()]);

        self
    }

    pub fn match_<A>(mut self, step: A) -> GraphTraversal<S, Map, T>
    where
        A: IntoMatchStep,
        T: Terminator<Map>,
    {
        self.bytecode
            .add_step(String::from("match"), step.into_step().take_params());
        GraphTraversal::new(self.terminator, self.bytecode)
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
