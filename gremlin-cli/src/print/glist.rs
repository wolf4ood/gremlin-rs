use crate::print;
use gremlin_client::List;
pub fn fmt(list: &List) -> String {
    format!(
        "[{}]",
        list.iter()
            .map(|value| format!("{}", print::fmt(value)))
            .collect::<Vec<String>>()
            .join(",")
    )
}
