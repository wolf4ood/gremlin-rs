use gremlin_client::{
    process::{traversal, GraphTraversalSource},
    GremlinClient,
};

fn main() -> Result<(), Box<std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let g = traversal().with_remote(client);

    chapter_32(&g)?;

    Ok(())
}

fn example<T>(
    source: &GraphTraversalSource,
    chapter: &str,
    description: &str,
    fun: T,
) -> Result<(), Box<std::error::Error>>
where
    T: Fn(&GraphTraversalSource) -> Result<String, Box<std::error::Error>>,
{
    println!("Chapter {} - {}", chapter, description);

    println!("===> {}", fun(source)?);

    println!("");

    Ok(())
}

fn chapter_32(g: &GraphTraversalSource) -> Result<(), Box<std::error::Error>> {
    let chapter = "3.2";

    example(&g, chapter, "Find vertices that are airports", |g| {
        let results = g.v(()).has_label("airport").to_list()?;
        Ok(format!("Found {} airports", results.len()))
    })?;

    example(&g, chapter, "Find the DFW vertex", |g| {
        let results = g.v(()).has("code", "DFW").to_list()?;
        Ok(format!("Found DPW airport with id {:?}", results[0].id()))
    })?;

    example(&g, chapter, "Combining those two previous queries", |g| {
        let results = g.v(()).has_label("airport").has("code", "DFW").to_list()?;
        Ok(format!("Found {} airports", results.len()))
    })?;

    Ok(())
}
