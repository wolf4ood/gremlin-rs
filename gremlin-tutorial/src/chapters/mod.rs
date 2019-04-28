use gremlin_client::process::GraphTraversalSource;

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
    ch32::all()
}
