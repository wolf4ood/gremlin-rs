use crate::GValue;

// pub type Set = Vec<GValue>;

#[derive(Debug, PartialEq, Clone)]
pub struct Set(Vec<GValue>);

impl Set {
    pub fn new(elements: Vec<GValue>) -> Self {
        Set(elements)
    }

    pub(crate) fn take(self) -> Vec<GValue> {
        self.0
    }
}

impl Into<Set> for Vec<GValue> {
    fn into(self) -> Set {
        Set(self)
    }
}

// impl std::ops::Index<usize> for List {
//     type Output = GValue;

//     fn index(&self, key: usize) -> &GValue {
//         self.0.get(key).expect("no entry found for key")
//     }
// }
