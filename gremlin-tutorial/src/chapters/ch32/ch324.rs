use crate::chapters::example;
use gremlin_client::process::traversal::GraphTraversalSource;
use gremlin_client::process::traversal::__;
use gremlin_client::structure::T;

pub fn chapter_324(g: &GraphTraversalSource) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.2.4";

    example(
        &g,
        chapter,
        "How many of each type of vertex are there?",
        |g| {
            let results = g.v(()).group_count().by(T::Label).to_list()?;
            Ok(format!("Found [{:?}] vertex types", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "How many of each type of vertex are there?",
        |g| {
            let results = g.v(()).label().group_count().to_list()?;
            Ok(format!("Found [{:?}] vertex types", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "How many of each type of edge are there?",
        |g| {
            let results = g.e(()).group_count().by(T::Label).to_list()?;
            Ok(format!("Found [{:?}] edge types", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "How many of each type of edge are there?",
        |g| {
            let results = g.e(()).label().group_count().to_list()?;
            Ok(format!("Found [{:?}] edge types", results[0]))
        },
    )?;

    example(&g, chapter, "As above but using group()", |g| {
        let results = g.v(()).group().by(T::Label).by(__.count()).to_list()?;
        Ok(format!("Found [{:?}] vertex types", results[0]))
    })?;

    example(
        &g,
        chapter,
        "How many airports are there in each country?",
        |g| {
            let results = g
                .v(())
                .has_label("airport")
                .group_count()
                .by("country")
                .to_list()?;
            Ok(format!(
                "Found [{:?}] airport in Italy",
                results[0]["IT"].get::<i64>()
            ))
        },
    )?;

    Ok(())
}
