use crate::structure::GValue;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq, Clone)]
pub struct Map(BTreeMap<String, GValue>);

impl From<HashMap<String, GValue>> for Map {
    fn from(val: HashMap<String, GValue>) -> Self {
        let map = val.into_iter().collect();
        Map(map)
    }
}

impl From<BTreeMap<String, GValue>> for Map {
    fn from(val: BTreeMap<String, GValue>) -> Self {
        Map(val)
    }
}

impl Map {
    pub(crate) fn remove(&mut self, key: &str) -> Option<GValue> {
        self.0.remove(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &GValue)> {
        self.0.iter()
    }

    pub fn get(&self, key: &str) -> Option<&GValue> {
        self.0.get(key)
    }
}

impl std::ops::Index<&str> for Map {
    type Output = GValue;

    fn index(&self, key: &str) -> &GValue {
        self.0.get(key).expect("no entry found for key")
    }
}
impl std::iter::FromIterator<(String, GValue)> for Map {
    fn from_iter<I: IntoIterator<Item = (String, GValue)>>(iter: I) -> Self {
        Map(iter.into_iter().collect())
    }
}
