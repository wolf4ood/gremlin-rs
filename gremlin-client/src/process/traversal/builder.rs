use crate::conversion::ToGValue;
use crate::process::traversal::step::by::IntoByStep;
use crate::process::traversal::step::dedup::DedupStep;
use crate::process::traversal::step::from::IntoFromStep;
use crate::process::traversal::step::has::IntoHasStep;
use crate::process::traversal::step::limit::LimitStep;
use crate::process::traversal::step::local::IntoLocalStep;
use crate::process::traversal::step::loops::LoopsStep;
use crate::process::traversal::step::match_step::IntoMatchStep;
use crate::process::traversal::step::not::IntoNotStep;
use crate::process::traversal::step::or::IntoOrStep;
use crate::process::traversal::step::repeat::IntoRepeatStep;
use crate::process::traversal::step::select::IntoSelectStep;
use crate::process::traversal::step::to::IntoToStep;
use crate::process::traversal::step::until::IntoUntilStep;
use crate::process::traversal::step::where_step::IntoWhereStep;

use crate::process::traversal::{Bytecode, Scope};
use crate::structure::Labels;
use crate::{structure::GIDs, structure::IntoPredicate, GValue};

#[derive(Clone)]
pub struct TraversalBuilder {
    pub(crate) bytecode: Bytecode,
}

impl Default for TraversalBuilder {
    fn default() -> Self {
        TraversalBuilder {
            bytecode: Bytecode::default(),
        }
    }
}

impl TraversalBuilder {
    pub fn new(bytecode: Bytecode) -> Self {
        TraversalBuilder { bytecode }
    }
    pub fn bytecode(&self) -> &Bytecode {
        &self.bytecode
    }

    pub fn v<T>(mut self, ids: T) -> TraversalBuilder
    where
        T: Into<GIDs>,
    {
        self.bytecode.add_step(
            String::from("V"),
            ids.into().0.iter().map(|id| id.to_gvalue()).collect(),
        );
        self
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

    pub fn add_v<A>(mut self, label: A) -> Self
    where
        A: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("addV"),
            label.into().0.into_iter().map(GValue::from).collect(),
        );

        self
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

    pub fn has_many<A>(mut self, steps: Vec<A>) -> Self
    where
        A: IntoHasStep,
    {
        for step in steps {
            self.bytecode
                .add_step(String::from("has"), step.into_step().take_params());
        }
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

    pub fn add_e<A>(mut self, label: A) -> Self
    where
        A: Into<String>,
    {
        self.bytecode
            .add_step(String::from("addE"), vec![label.into().into()]);

        self
    }

    pub fn out<A>(mut self, labels: A) -> Self
    where
        A: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("out"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        self
    }

    pub fn out_e<A>(mut self, labels: A) -> Self
    where
        A: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("outE"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        self
    }

    pub fn out_v(mut self) -> Self {
        self.bytecode.add_step(String::from("outV"), vec![]);

        self
    }
    pub fn in_<A>(mut self, labels: A) -> Self
    where
        A: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("in"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        self
    }

    pub fn in_e<A>(mut self, labels: A) -> Self
    where
        A: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("inE"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );

        self
    }

    pub fn in_v(mut self) -> Self {
        self.bytecode.add_step(String::from("inV"), vec![]);

        self
    }

    pub fn label(mut self) -> Self {
        self.bytecode.add_step(String::from("label"), vec![]);

        self
    }

    pub fn from<A>(mut self, step: A) -> Self
    where
        A: IntoFromStep,
    {
        self.bytecode
            .add_step(String::from("from"), step.into_step().take_params());

        self
    }

    pub fn to<A>(mut self, step: A) -> Self
    where
        A: IntoToStep,
    {
        self.bytecode
            .add_step(String::from("to"), step.into_step().take_params());

        self
    }

    pub fn properties<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("properties"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        self
    }

    pub fn property_map<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("propertyMap"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        self
    }

    pub fn values<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("values"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        self
    }

    pub fn value_map<L>(mut self, labels: L) -> Self
    where
        L: Into<Labels>,
    {
        self.bytecode.add_step(
            String::from("valueMap"),
            labels.into().0.into_iter().map(GValue::from).collect(),
        );
        self
    }

    pub fn count(mut self) -> Self {
        self.bytecode.add_step(String::from("count"), vec![]);
        self
    }

    pub fn group_count(mut self, key: Option<String>) -> Self {
        let mut params = vec![];

        if let Some(k) = key {
            params.push(k.into());
        }
        self.bytecode.add_step(String::from("groupCount"), params);
        self
    }

    pub fn group(mut self, key: Option<String>) -> Self {
        let mut params = vec![];

        if let Some(k) = key {
            params.push(k.into());
        }
        self.bytecode.add_step(String::from("group"), params);
        self
    }

    pub fn by<A>(mut self, step: A) -> Self
    where
        A: IntoByStep,
    {
        self.bytecode
            .add_step(String::from("by"), step.into_step().take_params());
        self
    }

    pub fn select<A>(mut self, step: A) -> Self
    where
        A: IntoSelectStep,
    {
        self.bytecode
            .add_step(String::from("select"), step.into_step().take_params());
        self
    }

    pub fn fold(mut self) -> Self {
        self.bytecode.add_step(String::from("fold"), vec![]);
        self
    }
    pub fn unfold(mut self) -> Self {
        self.bytecode.add_step(String::from("unfold"), vec![]);
        self
    }

    pub fn path(mut self) -> Self {
        self.bytecode.add_step(String::from("path"), vec![]);
        self
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

    pub fn sum<A>(mut self, scope: A) -> Self
    where
        A: Into<Scope>,
    {
        self.bytecode
            .add_step(String::from("sum"), vec![scope.into().into()]);

        self
    }

    pub fn max<A>(mut self, scope: A) -> Self
    where
        A: Into<Scope>,
    {
        self.bytecode
            .add_step(String::from("max"), vec![scope.into().into()]);

        self
    }

    pub fn mean<A>(mut self, scope: A) -> Self
    where
        A: Into<Scope>,
    {
        self.bytecode
            .add_step(String::from("mean"), vec![scope.into().into()]);

        self
    }

    pub fn min<A>(mut self, scope: A) -> Self
    where
        A: Into<Scope>,
    {
        self.bytecode
            .add_step(String::from("min"), vec![scope.into().into()]);

        self
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

    pub fn match_<A>(mut self, step: A) -> Self
    where
        A: IntoMatchStep,
    {
        self.bytecode
            .add_step(String::from("match"), step.into_step().take_params());
        self
    }

    pub fn drop(mut self) -> Self {
        self.bytecode.add_step(String::from("drop"), vec![]);
        self
    }

    pub fn or<A>(mut self, step: A) -> Self
    where
        A: IntoOrStep,
    {
        self.bytecode
            .add_step(String::from("or"), step.into_step().take_params());
        self
    }

    pub fn project<A>(mut self, step: A) -> Self
    where
        A: IntoSelectStep,
    {
        self.bytecode
            .add_step(String::from("project"), step.into_step().take_params());
        self
    }

    pub fn map<A>(mut self, step: A) -> Self
    where
        A: IntoByStep,
    {
        self.bytecode
            .add_step(String::from("map"), step.into_step().take_params());
        self
    }

    pub fn repeat<A>(mut self, step: A) -> Self
    where
        A: IntoRepeatStep,
    {
        self.bytecode
            .add_step(String::from("repeat"), step.into_step().take_params());

        self
    }

    pub fn until<A>(mut self, step: A) -> Self
    where
        A: IntoUntilStep,
    {
        self.bytecode
            .add_step(String::from("until"), step.into_step().take_params());

        self
    }

    pub fn simple_path(mut self) -> Self {
        self.bytecode.add_step(String::from("simplePath"), vec![]);

        self
    }

    pub fn sample(mut self, step: i32) -> Self {
        self.bytecode
            .add_step(String::from("sample"), vec![step.into()]);
        self
    }

    pub fn loops<A>(mut self, step: A) -> Self
    where
        A: Into<LoopsStep>,
    {
        self.bytecode
            .add_step(String::from("loops"), step.into().params());
        self
    }

    pub fn local<A>(mut self, step: A) -> Self
    where
        A: IntoLocalStep,
    {
        self.bytecode
            .add_step(String::from("local"), step.into_step().take_params());

        self
    }
}
