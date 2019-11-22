use crate::process::traversal::step::has::IntoHasStep;
use crate::process::traversal::step::not::IntoNotStep;
use crate::process::traversal::step::loops::LoopsStep;
use crate::process::traversal::TraversalBuilder;
use crate::structure::{GIDs, Labels};

pub struct AnonymousTraversalSource {
    traversal: TraversalBuilder,
}

impl AnonymousTraversalSource {
    pub fn new() -> AnonymousTraversalSource {
        AnonymousTraversalSource {
            traversal: TraversalBuilder::default(),
        }
    }

    pub fn v<T>(&self, ids: T) -> TraversalBuilder
    where
        T: Into<GIDs>,
    {
        self.traversal.clone().v(ids)
    }

    pub fn count(&self) -> TraversalBuilder {
        self.traversal.clone().count()
    }

    pub fn out<L>(&self, labels: L) -> TraversalBuilder
    where
        L: Into<Labels>,
    {
        self.traversal.clone().out(labels)
    }

    pub fn values<L>(&self, labels: L) -> TraversalBuilder
    where
        L: Into<Labels>,
    {
        self.traversal.clone().values(labels)
    }
    pub fn has_label<L>(&self, labels: L) -> TraversalBuilder
    where
        L: Into<Labels>,
    {
        self.traversal.clone().has_label(labels)
    }

    pub fn as_<A>(&self, alias: A) -> TraversalBuilder
    where
        A: Into<String>,
    {
        self.traversal.clone().as_(alias)
    }

    pub fn has<A>(&self, step: A) -> TraversalBuilder
    where
        A: IntoHasStep,
    {
        self.traversal.clone().has(step)
    }

    pub fn has_many<A>(&self, steps: Vec<A>) -> TraversalBuilder
    where
        A: IntoHasStep,
    {
        self.traversal.clone().has_many(steps)
    }

    pub fn not<A>(&self, step: A) -> TraversalBuilder
    where
        A: IntoNotStep,
    {
        self.traversal.clone().not(step)
    }

    pub fn loops<A>(&self, step: A) -> TraversalBuilder 
    where
        A: Into<LoopsStep>,
    {
        self.traversal.clone().loops(step)
    }
}

impl Default for AnonymousTraversalSource {
    fn default() -> Self {
        Self::new()
    }
}
