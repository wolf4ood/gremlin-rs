mod ch342;

use gremlin_client::process::traversal::GraphTraversalSource;

pub fn all() -> Vec<Box<Fn(&GraphTraversalSource) -> Result<(), Box<std::error::Error>>>> {
    vec![Box::new(ch342::chapter_342)]
}
