use gremlin_client::process::traversal::{GraphTraversalSource, SyncTerminator};

pub type TraversalExamples =
    Vec<Box<Fn(&GraphTraversalSource<SyncTerminator>) -> Result<(), Box<std::error::Error>>>>;

mod ch310;
mod ch311;
mod ch312;
mod ch314;
mod ch32;
mod ch33;
mod ch34;

fn example<T>(
    source: &GraphTraversalSource<SyncTerminator>,
    chapter: &str,
    description: &str,
    fun: T,
) -> Result<(), Box<std::error::Error>>
where
    T: Fn(&GraphTraversalSource<SyncTerminator>) -> Result<String, Box<std::error::Error>>,
{
    println!("Chapter {} - {}", chapter, description);

    println!("===> {}", fun(source)?);

    println!();

    Ok(())
}

pub fn all() -> TraversalExamples {
    let mut chapters = vec![];

    chapters.append(&mut ch32::all());
    chapters.append(&mut ch33::all());
    chapters.append(&mut ch34::all());
    chapters.append(&mut ch310::all());
    chapters.append(&mut ch311::all());
    chapters.append(&mut ch312::all());
    chapters.append(&mut ch314::all());
    chapters
}
