use crate::GValue;

#[derive(Debug, PartialEq, Clone)]
pub struct List(Vec<GValue>);

impl List {
    pub fn new(elements: Vec<GValue>) -> Self {
        List(elements)
    }

    pub(crate) fn take(self) -> Vec<GValue> {
        self.0
    }
}

impl Into<List> for Vec<GValue> {
    fn into(self) -> List {
        List(self)
    }
}

impl std::ops::Index<usize> for List {
    type Output = GValue;

    fn index(&self, key: usize) -> &GValue {
        self.0.get(key).expect("no entry found for key")
    }
}
