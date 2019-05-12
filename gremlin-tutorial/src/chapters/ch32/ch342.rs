use crate::chapters::example;
use gremlin_client::process::traversal::GraphTraversalSource;

pub fn chapter_342(g: &GraphTraversalSource) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.4.2";

    example(&g, chapter, "Removing duplcated with dedup", |g| {
        let results = g
            .v(())
            .has(("region", "GB-ENG"))
            .values("runways")
            .dedup(())
            .fold()
            .to_list()?;

        Ok(format!("Found [{:?}] unique elements", results[0]))
    })?;
    Ok(())
}
