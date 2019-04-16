pub struct Labels(pub(crate) Vec<String>);

impl Into<Labels> for &str {
    fn into(self) -> Labels {
        Labels(vec![String::from(self)])
    }
}

impl Into<Labels> for () {
    fn into(self) -> Labels {
        Labels(vec![])
    }
}
