macro_rules! g_serielizer {
    ($name:ident, { $($key:expr => $value:ident),*}) => {

        pub fn $name(val: &Value) -> GremlinResult<GValue> {
            if let Value::String(ref s) = val {
                Ok(s.clone().into())
            } else {
                let _type = &val["@type"];
                let _type = expect_string!(_type);
                let _value = &val["@value"];

                match _type {
                    $($key => {
                        $value(&$name,_value)
                    })*
                    _ => Err($crate::GremlinError::Json(format!("Type {} not supporterd",_type)))
                }
            }
        }
    };
}

macro_rules! expect_array {
    ($value:expr) => {
        match $value.as_array() {
            Some(v) => Ok(v),
            None => Err($crate::GremlinError::Json(String::from("Expected array"))),
        }?;
    };
}

macro_rules! expect_i32 {
    ($value:expr) => {
        match $value.as_i64() {
            Some(v) => Ok(v),
            None => Err($crate::GremlinError::Json(String::from("Expected i32"))),
        }? as i32;
    };
}

macro_rules! expect_i64 {
    ($value:expr) => {
        match $value.as_i64() {
            Some(v) => Ok(v),
            None => Err($crate::GremlinError::Json(String::from("Expected i64"))),
        }?;
    };
}
macro_rules! expect_float {
    ($value:expr) => {
        match $value.as_f64() {
            Some(v) => Ok(v),
            None => Err($crate::GremlinError::Json(String::from("Expected float"))),
        }? as f32;
    };
}
macro_rules! expect_double {
    ($value:expr) => {
        match $value.as_f64() {
            Some(v) => Ok(v),
            None => Err($crate::GremlinError::Json(String::from("Expected double"))),
        }?;
    };
}
macro_rules! expect_string {
    ($value:expr) => {
        match $value.as_str() {
            Some(v) => Ok(v),
            None => Err($crate::GremlinError::Json(format!(
                "Expected {} to be a String",
                $value
            ))),
        }?;
    };
}
