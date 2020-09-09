use super::gkey;
use crate::print;
use gremlin_client::Map;

pub fn fmt(map: &Map) -> String {
    format!(
        "[{}]",
        map.iter()
            .map(|(k, v)| format!("{}:{}", gkey::fmt(k), print::fmt(v)))
            .collect::<Vec<String>>()
            .join(",")
    )
}
