use crate::chapters::example;
use gremlin_client::process::traversal::GraphTraversalSource;
use gremlin_client::structure::P;

fn chapter_311(g: &GraphTraversalSource) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.11";

    example(&g, chapter, "// Airports with at least 5 runways", |g| {
        let results = g
            .v(())
            .has(("runways", P::gte(5)))
            .values(vec!["code", "runways"])
            .to_list()?;
        Ok(format!("Found {} airports", results.len()))
    })?;

    Ok(())
}

pub fn all() -> Vec<Box<Fn(&GraphTraversalSource) -> Result<(), Box<std::error::Error>>>> {
    vec![Box::new(chapter_311)]
}
