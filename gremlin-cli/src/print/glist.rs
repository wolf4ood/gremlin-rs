use crate::print;
use gremlin_client::List;

use anyhow::Result;

pub fn fmt(list: &List) -> Result<String> {
    Ok(format!(
        "[{}]",
        list.iter()
            .map(|value| print::fmt(value))
            .collect::<Result<Vec<String>>>()?
            .join(",")
    ))
}
