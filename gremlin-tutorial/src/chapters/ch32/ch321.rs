use crate::chapters::example;
use gremlin_client::process::GraphTraversalSource;

pub fn chapter_321(g: &GraphTraversalSource) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.2.1";

    example(
        &g,
        chapter,
        "What property values are stored in the DFW vertex?",
        |g| {
            let results = g
                .v(())
                .has(("airport", "code", "DFW"))
                .values(())
                .to_list()?;
            Ok(format!(
                "Found values [{}] ",
                results
                    .iter()
                    .map(|f| format!("{:?}", f))
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
        },
    )?;

    example(&g, chapter, "Return just the city name property", |g| {
        let results = g
            .v(())
            .has(("airport", "code", "DFW"))
            .values("city")
            .to_list()?;
        Ok(format!(
            "Found values [{}] ",
            results
                .iter()
                .map(|f| format!("{:?}", f))
                .collect::<Vec<String>>()
                .join(", ")
        ))
    })?;

    example(
        &g,
        chapter,
        "Return the 'runways' and 'icao' property values.",
        |g| {
            let results = g
                .v(())
                .has(("airport", "code", "DFW"))
                .values(vec!["runways", "icao"])
                .to_list()?;
            Ok(format!(
                "Found values [{}] ",
                results
                    .iter()
                    .map(|f| format!("{:?}", f))
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
        },
    )?;

    Ok(())
}
