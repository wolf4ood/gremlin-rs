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

    pub fn iter(&self) -> impl Iterator<Item = &GValue> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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
