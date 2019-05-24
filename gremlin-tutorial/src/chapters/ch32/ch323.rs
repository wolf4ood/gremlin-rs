use crate::chapters::example;
use gremlin_client::process::traversal::{GraphTraversalSource, SyncTerminator};

pub fn chapter_323(g: &GraphTraversalSource<SyncTerminator>) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.2.3";

    example(
        &g,
        chapter,
        "How many airports are there in the graph?",
        |g| {
            let results = g.v({}).has_label("airport").count().to_list()?;
            Ok(format!("Found [{}] airports", results[0]))
        },
    )?;

    example(&g, chapter, "How many routes are there?", |g| {
        let results = g
            .v({})
            .has_label("airport")
            .out_e("route")
            .count()
            .to_list()?;
        Ok(format!("Found [{}] routes", results[0]))
    })?;

    example(&g, chapter, "How many routes are there?", |g| {
        let results = g.v({}).out_e("route").count().to_list()?;
        Ok(format!("Found [{}] routes", results[0]))
    })?;

    example(&g, chapter, "How many routes are there?", |g| {
        let results = g.e({}).has_label("route").count().to_list()?;
        Ok(format!("Found [{}] routes", results[0]))
    })?;

    Ok(())
}
