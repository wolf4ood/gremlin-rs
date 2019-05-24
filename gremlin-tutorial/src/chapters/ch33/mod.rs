mod ch331;
mod ch332;
use crate::chapters::TraversalExamples;

pub fn all() -> TraversalExamples {
    vec![Box::new(ch331::chapter_331), Box::new(ch332::chapter_332)]
}
