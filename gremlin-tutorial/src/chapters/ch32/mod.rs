mod ch321;
mod ch322;
mod ch323;
mod ch324;

use gremlin_client::process::traversal::GraphTraversalSource;

use crate::chapters::example;

fn chapter_32(g: &GraphTraversalSource) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.2";

    example(&g, chapter, "Find vertices that are airports", |g| {
        let results = g.v(()).has_label("airport").to_list()?;
        Ok(format!("Found {} airports", results.len()))
    })?;

    example(&g, chapter, "Find the DFW vertex", |g| {
        let results = g.v(()).has(("code", "DFW")).to_list()?;
        Ok(format!("Found DPW airport with id {:?}", results[0].id()))
    })?;

    example(&g, chapter, "Combining those two previous queries", |g| {
        let results = g
            .v(())
            .has_label("airport")
            .has(("code", "DFW"))
            .to_list()?;
        Ok(format!("Found {} airports", results.len()))
    })?;

    Ok(())
}

pub fn all() -> Vec<Box<Fn(&GraphTraversalSource) -> Result<(), Box<std::error::Error>>>> {
    vec![
        Box::new(chapter_32),
        Box::new(ch321::chapter_321),
        Box::new(ch322::chapter_322),
        Box::new(ch323::chapter_323),
        Box::new(ch324::chapter_324),
    ]
}
