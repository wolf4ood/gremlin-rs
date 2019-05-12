use gremlin_client::process::traversal::GraphTraversalSource;

mod ch311;
mod ch32;

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

pub fn all() -> Vec<Box<Fn(&GraphTraversalSource) -> Result<(), Box<std::error::Error>>>> {
    let mut chapters = vec![];

    chapters.append(&mut ch32::all());
    chapters.append(&mut ch311::all());
    chapters
}
