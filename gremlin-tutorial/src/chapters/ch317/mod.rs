use crate::chapters::example;
use crate::chapters::TraversalExamples;
use gremlin_client::process::traversal::{GraphTraversalSource, SyncTerminator, __};
use gremlin_client::structure::P;

fn chapter_317(g: &GraphTraversalSource<SyncTerminator>) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.17";

    example(
        &g,
        chapter,
        "Find any cases of where you can fly from JFK non stop to a place you cannot get back from non stop",
        |g| {
            let results = g
                .v({})
                .has(("code","JFK"))
                .match_([__.as_("s").out({}).as_("d"),__.not(__.as_("d").out({}).as_("s"))])
                .select(["s","d"])
                .by("code")
                .to_list()?;
            Ok(format!("Found values {:?} ", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "How many occurrences of the pattern in the graph are there?",
        |g| {
            let results = g
                .v({})
                .has_label("airport")
                .match_([
                    __.as_("s").out({}).as_("d"),
                    __.not(__.as_("d").out({}).as_("s")),
                ])
                .count()
                .to_list()?;
            Ok(format!("Pattern count {:?} ", results[0]))
        },
    )?;

    example(
        &g,
        chapter,
        "All routes between two airports with one intermediate stop where there is no direct flight",
        |g| {
            let results = g
                .v({})
                .has_label("airport")
                .match_([
                    __.as_("a").out({}).as_("b"),
                    __.as_("b").out({}).where_(P::neq("a")).as_("c"),
                    __.not(__.as_("a").out({}).as_("c")),
                ])
                .select(["a","b","c"])
                .by("code")
                .limit(10)
                .to_list()?;
            Ok(format!("First element {:?} ", results[0]))
        },
    )?;

    Ok(())
}

pub fn all() -> TraversalExamples {
    vec![Box::new(chapter_317)]
}
