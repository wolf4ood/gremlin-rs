use gremlin_client::GKey;

pub fn fmt(gkey: &GKey) -> String {
    match gkey {
        GKey::String(s) => format!("{}", s),
        _ => todo!(),
    }
}
