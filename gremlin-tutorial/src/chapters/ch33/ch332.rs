use crate::chapters::example;
use gremlin_client::process::traversal::{GraphTraversalSource, SyncTerminator, __};

pub fn chapter_332(g: &GraphTraversalSource<SyncTerminator>) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.3.2";

    example(
        &g,
        chapter,
        "This time, for each route, return both vertices and the edge that connects them",
        |g| {
            let results = g
                .v({})
                .has(("airport", "code", "LCY"))
                .out_e({})
                .in_v()
                .path()
                .to_list()?;

            Ok(format!("Found [{:?}] path", results[0]))
        },
    )?;

    example(&g, chapter, "Same as above with by modulator", |g| {
        let results = g
            .v({})
            .has(("airport", "code", "LCY"))
            .out_e({})
            .in_v()
            .path()
            .by("code")
            .by("dist")
            .to_list()?;

        Ok(format!("Found [{:?}] path", results[0]))
    })?;

    example(
        &g,
        chapter,
        "Same as above with by modulator with longer form",
        |g| {
            let results = g
                .v({})
                .has(("airport", "code", "LCY"))
                .out_e({})
                .in_v()
                .path()
                .by("code")
                .by("dist")
                .by("code")
                .to_list()?;

            Ok(format!("Found [{:?}] path", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "Same as above with by modulator with city",
        |g| {
            let results = g
                .v({})
                .has(("airport", "code", "LCY"))
                .out_e({})
                .in_v()
                .path()
                .by("code")
                .by("dist")
                .by("city")
                .to_list()?;

            Ok(format!("Found [{:?}] path", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "Codes for the airports we visited along with a number of runways the second airport has.",
        |g| {
            let results = g
                .v({})
                .has(("airport", "code", "LCY"))
                .out({})
                .limit(5)
                .values("runways")
                .path()
                .by("code")
                .by("code")
                .by({})
                .to_list()?;

            Ok(format!("Found [{:?}] path", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "Finds five routes that start in Austin and creates a path result containing the airport code and city name for both the source and destination airports.",
        |g| {
            let results = g
                .v(3)
                .out({})
                .limit(5)
                .path()
                .by(__.values(vec!["code","city"]).fold())
                .to_list()?;

            Ok(format!("Found [{:?}] path", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        " Counts the number of outgoing routes for the source and destination airports as part of generating the path result.",
        |g| {
            let results = g
                .v(3)
                .out({})
                .limit(5)
                .path()
                .by(__.out({}).count().fold())
                .to_list()?;

            Ok(format!("Found [{:?}] path", results[0]))
        },
    )?;

    Ok(())
}
