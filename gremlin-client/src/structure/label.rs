pub enum LabelType {
    String_(String),
    Bool(bool),
}

pub struct Labels(pub(crate) Vec<LabelType>);

impl Into<Labels> for &str {
    fn into(self) -> Labels {
        Labels(vec![LabelType::String_(String::from(self))])
    }
}

impl Into<Labels> for () {
    fn into(self) -> Labels {
        Labels(vec![])
    }
}

impl Into<Labels> for Vec<&str> {
    fn into(self) -> Labels {
        Labels(
            self.into_iter()
                .map(|val| LabelType::String_(String::from(val)))
                .collect(),
        )
    }
}

impl Into<Labels> for Vec<String> {
    fn into(self) -> Labels {
        Labels(
            self.into_iter()
                .map(|val| LabelType::String_(val))
                .collect(),
        )
    }
}

impl Into<Labels> for bool {
    fn into(self) -> Labels {
        Labels(vec![LabelType::Bool(self)])
    }
}

macro_rules! impl_into_labels_str {
    ($n:expr) => {
        impl Into<Labels> for [&str; $n] {
            fn into(self) -> Labels {
                Labels(
                    self.iter()
                        .map(|s| LabelType::String_(String::from(*s)))
                        .collect(),
                )
            }
        }
    };
}

impl_into_labels_str!(1);
impl_into_labels_str!(2);
impl_into_labels_str!(3);
impl_into_labels_str!(4);
impl_into_labels_str!(5);
impl_into_labels_str!(6);
impl_into_labels_str!(7);
impl_into_labels_str!(8);
impl_into_labels_str!(9);
impl_into_labels_str!(10);

macro_rules! impl_into_labels_string {
    ($n:expr) => {
        impl Into<Labels> for [String; $n] {
            fn into(self) -> Labels {
                Labels(
                    self.iter()
                        .map(|val| LabelType::String_(val.clone()))
                        .collect(),
                )
            }
        }
    };
}

impl_into_labels_string!(1);
impl_into_labels_string!(2);
impl_into_labels_string!(3);
impl_into_labels_string!(4);
impl_into_labels_string!(5);
impl_into_labels_string!(6);
impl_into_labels_string!(7);
impl_into_labels_string!(8);
impl_into_labels_string!(9);
impl_into_labels_string!(10);
