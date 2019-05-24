use crate::chapters::example;
use crate::chapters::TraversalExamples;
use gremlin_client::process::traversal::{GraphTraversalSource, SyncTerminator, __};
use gremlin_client::structure::P;

fn chapter_314(g: &GraphTraversalSource<SyncTerminator>) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.14";

    example(
        &g,
        chapter,
        "Find airports with more than five runways",
        |g| {
            let results = g
                .v({})
                .where_(__.values("runways").is(P::gt(5)))
                .to_list()?;
            Ok(format!("Found {:?} airports", results.len()))
        },
    )?;

    example(
        &g,
        chapter,
        "Airports with more than 60 unique routes from them",
        |g| {
            let results = g
                .v({})
                .has_label("airport")
                .where_(__.out("route").count().is(P::gt(60)))
                .count()
                .to_list()?;

            Ok(format!("Found {:?} airports", results[0]))
        },
    )?;

    Ok(())
}

pub fn all() -> TraversalExamples {
    vec![Box::new(chapter_314)]
}
