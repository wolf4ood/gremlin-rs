use super::glist;
use anyhow::Result;
use gremlin_client::Path;

pub fn fmt(path: &Path) -> Result<String> {
    glist::fmt(path.objects())
}
