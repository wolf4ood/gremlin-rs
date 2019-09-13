use crate::chapters::example;
use gremlin_client::process::traversal::__;
use gremlin_client::process::traversal::{GraphTraversalSource, SyncTerminator};

pub fn chapter_322(g: &GraphTraversalSource<SyncTerminator>) -> Result<(), Box<dyn std::error::Error>> {
    let chapter = "3.2.2";

    example(
        &g,
        chapter,
        "Find all edges that have a 'dist' property",
        |g| {
            let property = "dist";
            let results = g.e(()).has(property).to_list()?;
            Ok(format!(
                "Found [{}] edges with property {}",
                results.len(),
                property
            ))
        },
    )?;

    example(
        &g,
        chapter,
        "Find all vertices that have a 'region' property",
        |g| {
            let property = "region";
            let results = g.v(()).has(property).to_list()?;
            Ok(format!(
                "Found [{}] vertices with property {}",
                results.len(),
                property
            ))
        },
    )?;

    example(
        &g,
        chapter,
        "Find all the vertices that do not have a 'region' property",
        |g| {
            let property = "region";
            let results = g.v(()).has_not(property).to_list()?;
            Ok(format!(
                "Found [{}] vertices without property {}",
                results.len(),
                property
            ))
        },
    )?;

    example(&g, chapter, "The above is shorthand for", |g| {
        let property = "region";
        let results = g.v(()).not(__.has("region")).to_list()?;
        Ok(format!(
            "Found [{}] vertices without property {}",
            results.len(),
            property
        ))
    })?;

    Ok(())
}
