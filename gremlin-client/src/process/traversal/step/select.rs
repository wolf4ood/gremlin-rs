use crate::process::traversal::TraversalBuilder;
use crate::structure::{GValue, Pop};

pub struct SelectStep {
    params: Vec<GValue>,
}

impl SelectStep {
    fn new(params: Vec<GValue>) -> Self {
        SelectStep { params }
    }
}

impl SelectStep {
    pub fn take_params(self) -> Vec<GValue> {
        self.params
    }
}

pub trait IntoSelectStep {
    fn into_step(self) -> SelectStep;
}

impl IntoSelectStep for &str {
    fn into_step(self) -> SelectStep {
        SelectStep::new(vec![String::from(self).into()])
    }
}

impl IntoSelectStep for Pop {
    fn into_step(self) -> SelectStep {
        SelectStep::new(vec![GValue::Pop(self)])
    }
}

impl IntoSelectStep for Vec<&str> {
    fn into_step(self) -> SelectStep {
        SelectStep::new(self.into_iter().map(GValue::from).collect())
    }
}

impl IntoSelectStep for TraversalBuilder {
    fn into_step(self) -> SelectStep {
        SelectStep::new(vec![self.bytecode.into()])
    }
}

impl<B> IntoSelectStep for (Pop, B)
where
    B: Into<GValue>,
{
    fn into_step(self) -> SelectStep {
        SelectStep::new(vec![GValue::Pop(self.0), self.1.into()])
    }
}

macro_rules! impl_into_select {
    ($n:expr) => {
        impl<T: Clone> IntoSelectStep for [T; $n]
        where
            T: Into<String>,
        {
            fn into_step(self) -> SelectStep {
                SelectStep::new(self.iter().map(|e| e.clone().into().into()).collect())
            }
        }
    };
}

impl_into_select!(1);
impl_into_select!(2);
impl_into_select!(3);
impl_into_select!(4);
impl_into_select!(5);
impl_into_select!(6);
impl_into_select!(7);
impl_into_select!(8);
impl_into_select!(9);
impl_into_select!(10);
