use crate::chapters::example;
use gremlin_client::process::traversal::{GraphTraversalSource, Order, SyncTerminator, __};

use crate::chapters::TraversalExamples;

fn chapter_312(g: &GraphTraversalSource<SyncTerminator>) -> Result<(), Box<dyn std::error::Error>> {
    let chapter = "3.12";

    example(
        &g,
        chapter,
        "Sort the first 20 airports returned in ascending order",
        |g| {
            let results = g
                .v(())
                .has_label("airport")
                .limit(20)
                .values("code")
                .order(())
                .fold()
                .to_list()?;
            Ok(format!("Found {:?} ", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "Sort all of the airports in the graph by their code and then return the first 20",
        |g| {
            let results = g
                .v(())
                .has_label("airport")
                .order(())
                .by("code")
                .limit(20)
                .values("code")
                .fold()
                .to_list()?;

            Ok(format!("Found {:?} ", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "Sort all of the places you can fly to from Austin (AUS)",
        |g| {
            let results = g
                .v(())
                .has(("code", "AUS"))
                .out(())
                .order(())
                .by("code")
                .values(["code", "icao"])
                .fold()
                .to_list()?;

            Ok(format!("Found {:?} ", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "Sort the first 20 airports returned in descending order",
        |g| {
            let results = g
                .v(())
                .has_label("airport")
                .limit(20)
                .values("code")
                .order(())
                .by(Order::Desc)
                .fold()
                .to_list()?;

            Ok(format!("Found {:?} ", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "Sort the first 20 airports returned in shuffle order",
        |g| {
            let results = g
                .v(())
                .has_label("airport")
                .limit(20)
                .values("code")
                .order(())
                .by(Order::Shuffle)
                .fold()
                .to_list()?;

            Ok(format!("Found {:?} ", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "List the 10 airports with the longest runways in decreasing order.",
        |g| {
            let results = g
                .v(())
                .has_label("airport")
                .order(())
                .by(("longest", Order::Shuffle))
                .value_map(())
                .select(["code", "longest"])
                .limit(10)
                .to_list()?;

            Ok(format!("Found {:?} ", results))
        },
    )?;

    example(
        &g,
        chapter,
        "sorts all the airports by longest runway in descending order and returns the valueMap for the first of those",
        |g| {
            let results = g
                .v(())
                .has_label("airport")
                .order(())
                .by((__.values("longest"), Order::Shuffle))
                .limit(1)
                .value_map(())
                .to_list()?;

            Ok(format!("Found {:?} ", results[0]))
        },
    )?;

    Ok(())
}

pub fn all() -> TraversalExamples {
    vec![Box::new(chapter_312)]
}
