use super::gkey;
use crate::print;
use anyhow::Result;
use gremlin_client::Map;

pub fn fmt(map: &Map) -> Result<String> {
    Ok(format!(
        "[{}]",
        map.iter()
            .map(|(k, v)| Ok(format!("{}:{}", gkey::fmt(k), print::fmt(v)?)))
            .collect::<Result<Vec<String>>>()?
            .join(",")
    ))
}
