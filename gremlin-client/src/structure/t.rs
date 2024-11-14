use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum T {
    Id,
    Key,
    Label,
    Value,
}

impl TryFrom<&str> for T {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "id" => Ok(T::Id),
            "key" => Ok(T::Key),
            "label" => Ok(T::Label),
            "value" => Ok(T::Value),
            other => Err(format!("Unknown T literal {other:?}")),
        }
    }
}
