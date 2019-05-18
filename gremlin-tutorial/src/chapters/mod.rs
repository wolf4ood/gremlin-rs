use gremlin_client::process::traversal::GraphTraversalSource;

mod ch310;
mod ch311;
mod ch314;
mod ch32;
mod ch33;
mod ch34;

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
    chapters.append(&mut ch33::all());
    chapters.append(&mut ch34::all());
    chapters.append(&mut ch310::all());
    chapters.append(&mut ch311::all());
    chapters.append(&mut ch314::all());
    chapters
}
