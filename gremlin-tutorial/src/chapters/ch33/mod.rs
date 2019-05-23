mod ch331;
mod ch332;
use gremlin_client::process::traversal::{GraphTraversalSource, SyncTerminator};

pub fn all(
) -> Vec<Box<Fn(&GraphTraversalSource<SyncTerminator>) -> Result<(), Box<std::error::Error>>>> {
    vec![Box::new(ch331::chapter_331), Box::new(ch332::chapter_332)]
}
