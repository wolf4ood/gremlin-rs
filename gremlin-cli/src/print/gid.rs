use gremlin_client::GID;

pub fn fmt(gid: &GID) -> String {
    match gid {
        GID::Int32(i) => format!("{}", i),
        GID::String(s) => s.to_string(),
        GID::Int64(i) => format!("{}", i),
    }
}
