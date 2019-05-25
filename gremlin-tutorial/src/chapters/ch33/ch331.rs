use crate::chapters::example;
use gremlin_client::process::traversal::{GraphTraversalSource, SyncTerminator};

pub fn chapter_331(g: &GraphTraversalSource<SyncTerminator>) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.3.1";

    example(&g, chapter, "Where can I fly to from Austin?", |g| {
        let results = g
            .v(())
            .has(("airport", "code", "AUS"))
            .out(())
            .values("code")
            .fold()
            .to_list()?;
        Ok(format!("Found [{:?}] destinations", results[0]))
    })?;

    example(&g, chapter, "Where can I fly to from Austin?", |g| {
        let results = g
            .v(())
            .has(("airport", "code", "AUS"))
            .out("route")
            .values("code")
            .fold()
            .to_list()?;
        Ok(format!("Found [{:?}] destinations", results[0]))
    })?;

    example(
        &g,
        chapter,
        "Where can I fly to from Austin, with one stop on the way?",
        |g| {
            let results = g
                .v(())
                .has(("airport", "code", "AUS"))
                .out("route")
                .out("route")
                .values("code")
                .to_list()?;
            Ok(format!("Found [{:?}] destinations", results.len()))
        },
    )?;

    example(&g, chapter, "What routes come in to LCY?", |g| {
        let results = g
            .v(())
            .has(("airport", "code", "LCY"))
            .in_("route")
            .values("code")
            .to_list()?;
        Ok(format!("Found [{:?}] destinations", results.len()))
    })?;

    example(
        &g,
        chapter,
        "Flights from London Heathrow (LHR) to airports in the USA",
        |g| {
            let results = g
                .v(())
                .has(("code", "LHR"))
                .out("route")
                .has(("country", "US"))
                .values("code")
                .to_list()?;
            Ok(format!(
                "Found [{:?}] US destinations from LHR",
                results.len()
            ))
        },
    )?;

    Ok(())
}
