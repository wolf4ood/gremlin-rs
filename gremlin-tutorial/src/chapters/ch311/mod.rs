use crate::chapters::example;
use gremlin_client::process::traversal::GraphTraversalSource;
use gremlin_client::structure::P;

fn chapter_311(g: &GraphTraversalSource) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.11";

    example(&g, chapter, "Airports with at least 5 runways", |g| {
        let results = g
            .v(())
            .has(("runways", P::gte(5)))
            .values(vec!["code", "runways"])
            .fold()
            .to_list()?;
        Ok(format!("Found {:?} ", results[0]))
    })?;

    example(&g, chapter, "Airports with less than 3 runways", |g| {
        let results = g
            .v(())
            .has(("runways", P::lt(2)))
            .values(vec!["code", "runways"])
            .fold()
            .to_list()?;
        Ok(format!("Found {:?} airports", results[0].len()))
    })?;

    example(&g, chapter, "How many airports have 3 runways?", |g| {
        let results = g.v(()).has(("runways", P::eq(3))).count().to_list()?;
        Ok(format!("Found {:?} airports", results[0]))
    })?;

    example(
        &g,
        chapter,
        "How many airports have anything but just 1 runway?",
        |g| {
            let results = g.v(()).has(("runways", P::neq(1))).count().to_list()?;
            Ok(format!("Found {:?} airports", results[0]))
        },
    )?;

    example(&g, chapter, " Airports with 1,2 or 3 runways.", |g| {
        let results = g
            .v(())
            .has(("runways", P::within(vec![1, 2, 3])))
            .count()
            .to_list()?;
        Ok(format!("Found {:?} airports", results[0]))
    })?;
    Ok(())
}

pub fn all() -> Vec<Box<Fn(&GraphTraversalSource) -> Result<(), Box<std::error::Error>>>> {
    vec![Box::new(chapter_311)]
}
