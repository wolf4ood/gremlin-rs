use crate::chapters::example;
use gremlin_client::process::traversal::GraphTraversalSource;

pub fn chapter_332(g: &GraphTraversalSource) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.3.2";

    example(
        &g,
        chapter,
        "This time, for each route, return both vertices and the edge that connects them",
        |g| {
            let results = g
                .v(())
                .has(("airport", "code", "LCY"))
                .out_e(())
                .in_v()
                .path()
                .to_list()?;

            Ok(format!("Found [{:?}] path", results[0]))
        },
    )?;

    example(&g, chapter, "Same as above with by modulator", |g| {
        let results = g
            .v(())
            .has(("airport", "code", "LCY"))
            .out_e(())
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
                .v(())
                .has(("airport", "code", "LCY"))
                .out_e(())
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
                .v(())
                .has(("airport", "code", "LCY"))
                .out_e(())
                .in_v()
                .path()
                .by("code")
                .by("dist")
                .by("city")
                .to_list()?;

            Ok(format!("Found [{:?}] path", results[0]))
        },
    )?;

    Ok(())
}
