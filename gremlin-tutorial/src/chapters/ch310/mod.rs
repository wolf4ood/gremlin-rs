use crate::chapters::example;
use gremlin_client::process::traversal::GraphTraversalSource;

fn chapter_310(g: &GraphTraversalSource) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.10";

    example(
        &g,
        chapter,
        "Sum of values - total runways of all airports",
        |g| {
            let results = g
                .v(())
                .has_label("airport")
                .values("runways")
                .sum(())
                .to_list()?;
            Ok(format!(
                "Total runways {:?} ",
                results[0].get::<i64>().unwrap()
            ))
        },
    )?;

    example(
        &g,
        chapter,
        "Statistical mean (average) value - average number of runways per airport",
        |g| {
            let results = g
                .v(())
                .has_label("airport")
                .values("runways")
                .mean(())
                .to_list()?;
            Ok(format!(
                "Average runways {:?} ",
                results[0].get::<f64>().unwrap()
            ))
        },
    )?;

    example(&g, chapter, "Maximum value - longest runway", |g| {
        let results = g
            .v(())
            .has_label("airport")
            .values("longest")
            .max(())
            .to_list()?;
        Ok(format!(
            "Max longest runway {:?} ",
            results[0].get::<i32>().unwrap()
        ))
    })?;

    Ok(())
}

pub fn all() -> Vec<Box<Fn(&GraphTraversalSource) -> Result<(), Box<std::error::Error>>>> {
    vec![Box::new(chapter_310)]
}
