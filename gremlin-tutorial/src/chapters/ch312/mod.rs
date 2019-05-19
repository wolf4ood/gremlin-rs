use crate::chapters::example;
use gremlin_client::process::traversal::GraphTraversalSource;

fn chapter_312(g: &GraphTraversalSource) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.12";

    example(
        &g,
        chapter,
        " Sort the first 20 airports returned in ascending order",
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

    Ok(())
}

pub fn all() -> Vec<Box<Fn(&GraphTraversalSource) -> Result<(), Box<std::error::Error>>>> {
    vec![Box::new(chapter_312)]
}
