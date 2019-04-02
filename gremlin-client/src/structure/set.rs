use crate::GValue;

// pub type Set = Vec<GValue>;

#[derive(Debug, PartialEq, Clone)]
pub struct Set(Vec<GValue>);

impl Set {
    pub(crate) fn take(self) -> Vec<GValue> {
        self.0
    }

    pub fn iter(&self) -> impl Iterator<Item = &GValue> {
        self.0.iter()
    }
}

impl Into<Set> for Vec<GValue> {
    fn into(self) -> Set {
        Set(self)
    }
}
