//! GraphSON V1 [docs](http://tinkerpop.apache.org/docs/current/dev/io/)
//!

use crate::structure::{
    Edge, GKey, GValue, IntermediateRepr, List, Map, Metric, Path, Property, TraversalExplanation,
    TraversalMetrics, Vertex, VertexProperty, GID,
};
use crate::GremlinError;
use crate::GremlinResult;
use serde_json::Value;
use std::collections::HashMap;

static G_METRICS: &'static str = "g:Metrics";
static G_TRAVERSAL_EXPLANATION: &'static str = "g:TraversalExplanation";
static G_TRAVERSAL_METRICS: &'static str = "g:TraversalMetrics";

// Deserialize a JSON value to a GID
pub fn deserialize_id<T>(reader: &T, val: &Value) -> GremlinResult<GID>
where
    T: Fn(&Value) -> GremlinResult<GValue>,
{
    match reader(val) {
        Ok(result) => match result {
            GValue::String(d) => Ok(GID::String(d)),
            GValue::Int32(d) => Ok(GID::Int32(d)),
            GValue::Int64(d) => Ok(GID::Int64(d)),
            _ => Err(GremlinError::Json(format!("{} cannot be an id", val))),
        },
        Err(e) => match e {
            GremlinError::Json(_e) => Ok(GID::String(val.to_string())),
            _ => Err(e),
        },
    }
}

pub fn deserialize_list<T>(reader: &T, val: &Value) -> GremlinResult<GValue>
where
    T: Fn(&Value) -> GremlinResult<GValue>,
{
    let val = get_value!(val, Value::Array)?;
    let mut elements = Vec::with_capacity(val.len());
    for item in val {
        elements.push(reader(item)?)
    }
    Ok(elements.into())
}

pub fn deserialize_map<T>(reader: &T, val: &Value) -> GremlinResult<GValue>
where
    T: Fn(&Value) -> GremlinResult<GValue>,
{
    let val = get_value!(val, Value::Object)?;
    let mut map = HashMap::new();
    for (k, v) in val {
        map.insert(GKey::String(k.to_string()), reader(v)?);
    }
    Ok(map.into())
}

// Vertex deserializer [docs](http://tinkerpop.apache.org/docs/3.4.6/dev/io/#_vertex)
pub fn deserialize_vertex<T>(reader: &T, val: &Value) -> GremlinResult<GValue>
where
    T: Fn(&Value) -> GremlinResult<GValue>,
{
    let label = val
        .get("label")
        .map(|f| get_value!(f, Value::String).map(Clone::clone))
        .unwrap_or_else(|| Ok(String::from("vertex")))?;

    let id = deserialize_id(reader, &val["id"])?;

    Ok(Vertex::new(
        id,
        label,
        deserialize_vertex_properties(reader, &val["properties"])?,
    )
    .into())
}

// Edge deserializer [docs](http://tinkerpop.apache.org/docs/3.4.6/dev/io/#_edge)
pub fn deserialize_edge<T>(reader: &T, val: &Value) -> GremlinResult<GValue>
where
    T: Fn(&Value) -> GremlinResult<GValue>,
{
    let label = val
        .get("label")
        .map(|f| get_value!(f, Value::String).map(Clone::clone))
        .unwrap_or_else(|| Ok(String::from("edge")))?;

    let id = deserialize_id(reader, &val["id"])?;

    let in_v_id = deserialize_id(reader, &val["inV"])?;
    let in_v_label = get_value!(&val["inVLabel"], Value::String)?.clone();

    let out_v_id = deserialize_id(reader, &val["outV"])?;
    let out_v_label = get_value!(&val["outVLabel"], Value::String)?.clone();

    Ok(Edge::new(
        id,
        label,
        in_v_id,
        in_v_label,
        out_v_id,
        out_v_label,
        HashMap::new(),
    )
    .into())
}

// Path deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_path_3)
pub fn deserialize_path<T>(reader: &T, val: &Value) -> GremlinResult<GValue>
where
    T: Fn(&Value) -> GremlinResult<GValue>,
{
    let labels = reader(&val["labels"])?;

    let objects = reader(&val["objects"])?.take::<List>()?;

    Ok(Path::new(labels, objects).into())
}

// Traversal Metrics deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_traversalmetrics)
pub fn deserialize_metrics<T>(reader: &T, val: &Value) -> GremlinResult<GValue>
where
    T: Fn(&Value) -> GremlinResult<GValue>,
{
    let mut metrics = deserialize_map(reader, &val)?.take::<Map>()?;

    let duration = remove_or_else(&mut metrics, "dur", G_TRAVERSAL_METRICS)?.take::<f64>()?;

    let m = remove_or_else(&mut metrics, "metrics", G_TRAVERSAL_METRICS)?
        .take::<List>()?
        .take()
        .drain(0..)
        .map(|e| e.take::<Metric>())
        .filter_map(Result::ok)
        .collect();

    Ok(TraversalMetrics::new(duration, m).into())
}

// Metrics deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_metrics)
pub fn deserialize_metric<T>(reader: &T, val: &Value) -> GremlinResult<GValue>
where
    T: Fn(&Value) -> GremlinResult<GValue>,
{
    let mut metric = deserialize_map(reader, &val)?.take::<Map>()?;

    let duration = remove_or_else(&mut metric, "dur", G_METRICS)?.take::<f64>()?;
    let id = remove_or_else(&mut metric, "id", G_METRICS)?.take::<String>()?;
    let name = remove_or_else(&mut metric, "name", G_METRICS)?.take::<String>()?;

    let mut counts = remove_or_else(&mut metric, "counts", G_METRICS)?.take::<Map>()?;
    let traversers = remove_or_else(&mut counts, "traverserCount", G_METRICS)?.take::<i64>()?;
    let count = remove_or_else(&mut counts, "elementCount", G_METRICS)?.take::<i64>()?;

    let mut annotations = remove(&mut metric, "annotations", G_METRICS)
        .map(|e| e.take::<Map>())
        .unwrap_or_else(|| Ok(Map::empty()))?;

    let perc_duration = remove(&mut annotations, "percentDur", G_METRICS)
        .map(|e| e.take::<f64>())
        .unwrap_or_else(|| Ok(0.0))?;

    let nested: GremlinResult<Vec<Metric>> = remove(&mut metric, "metrics", G_METRICS)
        .map(|e| e.take::<List>())
        .unwrap_or_else(|| Ok(List::new(vec![])))?
        .take()
        .into_iter()
        .map(|e| e.take::<Metric>())
        .collect();

    Ok(Metric::new(
        id,
        name,
        duration,
        count,
        traversers,
        perc_duration,
        nested?,
    )
    .into())
}

pub fn deserialize_explain<T>(reader: &T, val: &Value) -> GremlinResult<GValue>
where
    T: Fn(&Value) -> GremlinResult<GValue>,
{
    let mut explain = deserialize_map(reader, &val)?.take::<Map>()?;

    let original = remove_or_else(&mut explain, "original", G_TRAVERSAL_EXPLANATION)?
        .take::<List>()?
        .take()
        .drain(0..)
        .map(|s| s.take::<String>())
        .filter_map(Result::ok)
        .collect();

    let finals = remove_or_else(&mut explain, "final", G_TRAVERSAL_EXPLANATION)?
        .take::<List>()?
        .take()
        .drain(0..)
        .map(|s| s.take::<String>())
        .filter_map(Result::ok)
        .collect();

    let intermediate = remove_or_else(&mut explain, "intermediate", G_TRAVERSAL_EXPLANATION)?
        .take::<List>()?
        .take()
        .drain(0..)
        .map(|s| s.take::<Map>())
        .filter_map(Result::ok)
        .map(map_intermediate)
        .filter_map(Result::ok)
        .collect();

    Ok(TraversalExplanation::new(original, finals, intermediate).into())
}

fn map_intermediate(mut m: Map) -> GremlinResult<IntermediateRepr> {
    let traversal = remove_or_else(&mut m, "traversal", G_TRAVERSAL_EXPLANATION)?
        .take::<List>()?
        .take()
        .drain(0..)
        .map(|s| s.take::<String>())
        .filter_map(Result::ok)
        .collect();

    let strategy = remove_or_else(&mut m, "strategy", G_TRAVERSAL_EXPLANATION)?.take::<String>()?;

    let category = remove_or_else(&mut m, "category", G_TRAVERSAL_EXPLANATION)?.take::<String>()?;

    Ok(IntermediateRepr::new(traversal, strategy, category))
}

// Vertex Property deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_vertexproperty_3)
pub fn deserialize_vertex_property<T>(reader: &T, val: &Value) -> GremlinResult<GValue>
where
    T: Fn(&Value) -> GremlinResult<GValue>,
{
    let label = val
        .get("label")
        .map(|f| get_value!(f, Value::String).map(Clone::clone))
        .unwrap_or_else(|| Ok(String::from("vertex_property")))?;

    let id = deserialize_id(reader, &val["id"])?;
    let v = reader(&val["value"])?;
    Ok(VertexProperty::new(id, label, v).into())
}

// Property deserializer [docs](http://tinkerpop.apache.org/docs/current/dev/io/#_property_3)
pub fn deserialize_property<T>(reader: &T, val: &Value) -> GremlinResult<GValue>
where
    T: Fn(&Value) -> GremlinResult<GValue>,
{
    let label = val
        .get("key")
        .map(|f| get_value!(f, Value::String).map(Clone::clone))
        .unwrap_or_else(|| Ok(String::from("property")))?;

    let v = reader(&val["value"])?;
    Ok(Property::new(label, v).into())
}

pub fn deserialize_number<T>(_: &T, val: &Value) -> GremlinResult<GValue>
where
    T: Fn(&Value) -> GremlinResult<GValue>,
{
    if let Some(_i) = val.as_i64() {
        Ok(GValue::from(expect_i64!(val)))
    } else {
        Ok(GValue::from(expect_double!(val)))
    }
}

// deserialzer v1
pub fn deserializer_v1(val: &Value) -> GremlinResult<GValue> {
    let retval = match val {
        Value::Null => Err(GremlinError::Json(format!("Val {:#?} not supported.", val))),
        Value::Bool(b) => Ok(GValue::Bool(*b)),
        Value::Number(_n) => deserialize_number(&deserializer_v1, val),
        Value::String(s) => Ok(GValue::String(s.clone())),
        Value::Array(_a) => deserialize_list(&deserializer_v1, val),
        Value::Object(o) => {
            if let Some(Value::String(t)) = o.get("type") {
                match t.as_str() {
                    "edge" => deserialize_edge(&deserializer_v1, val),
                    "vertex" => deserialize_vertex(&deserializer_v1, val),
                    _ => Err(GremlinError::Json(format!("Val {:#?} not supported.", val))),
                }
            } else if o.contains_key("dur") && o.contains_key("id") {
                deserialize_metric(&deserializer_v1, val)
            } else if o.contains_key("dur") && o.contains_key("metrics") {
                deserialize_metrics(&deserializer_v1, val)
            } else if o.contains_key("final")
                && o.contains_key("intermediate")
                && o.contains_key("original")
            {
                deserialize_explain(&deserializer_v1, val)
            } else if o.contains_key("id") && o.contains_key("value") {
                deserialize_vertex_property(&deserializer_v1, val)
            } else if o.contains_key("key") && o.contains_key("value") {
                deserialize_property(&deserializer_v1, val)
            } else if o.contains_key("labels") && o.contains_key("objects") {
                deserialize_path(&deserializer_v1, val)
            } else {
                deserialize_map(&deserializer_v1, val)
            }
        }
    };

    retval
}

/*
g_serializer_1!(deserializer_v1, {
    "g:Int32" => deserialize_g32,
    "g:Int64" => deserialize_g64,
    "g:Float" => deserialize_f32,
    "g:Double" => deserialize_f64,
    "g:Date" => deserialize_date,
    "g:UUID" => deserialize_uuid,
    "g:List" => deserialize_list,
    "g:Map" => deserialize_map,
    "g:T" => deserialize_token,
    "g:Vertex" => deserialize_vertex,
    "g:VertexProperty" => deserialize_vertex_property,
    "g:Property" => deserialize_property,
    "g:Edge" => deserialize_edge,
    "g:Path" => deserialize_path,
    "g:TraversalMetrics" => deserialize_metrics,
    "g:Metrics" => deserialize_metric,
    "g:TraversalExplanation" => deserialize_explain,
    "g:Traverser" => deserialize_traverser
});
*/

fn deserialize_vertex_properties<T>(
    reader: &T,
    properties: &Value,
) -> GremlinResult<HashMap<String, Vec<VertexProperty>>>
where
    T: Fn(&Value) -> GremlinResult<GValue>,
{
    match properties {
        Value::Object(o) => {
            let mut p = HashMap::new();
            for (k, v) in o {
                match v {
                    Value::Array(arr) => {
                        let mut vec = vec![];
                        for elem in arr {
                            vec.push(reader(elem)?.take()?);
                        }
                        p.insert(k.clone(), vec);
                    }
                    _ => {
                        return Err(GremlinError::Json(format!(
                            "Expected object or null for properties. Found {}",
                            properties
                        )));
                    }
                };
            }
            Ok(p)
        }

        Value::Null => Ok(HashMap::new()),
        _ => Err(GremlinError::Json(format!(
            "Expected object or null for properties. Found {}",
            properties
        ))),
    }
}

fn remove_or_else(map: &mut Map, field: &str, owner: &str) -> GremlinResult<GValue> {
    remove(map, field, owner)
        .ok_or_else(|| GremlinError::Json(format!("Field {} not found in {}", field, owner)))
}

fn remove(map: &mut Map, field: &str, _owner: &str) -> Option<GValue> {
    map.remove(field)
}
// TESTS
#[cfg(test)]
mod tests {

    use super::deserializer_v1;
    use serde_json::json;

    use crate::structure::GValue;
    use std::collections::HashMap;

    #[test]
    fn test_collections() {
        // List
        let value = json!([1, 2, "3"]);

        let result = deserializer_v1(&value).expect("Failed to deserialize a List");

        assert_eq!(
            result,
            GValue::List(
                vec![
                    GValue::Int64(1),
                    GValue::Int64(2),
                    GValue::String(String::from("3")),
                ]
                .into()
            )
        );

        // Map

        let value = json!({
            "a": 1,
            "b": "marko"
        });

        let result = deserializer_v1(&value).expect("Failed to deserialize a Map");

        let mut map = HashMap::new();
        map.insert(String::from("a"), GValue::Int64(1));
        map.insert(String::from("b"), GValue::String(String::from("marko")));
        assert_eq!(result, GValue::from(map));
    }

    #[test]
    fn test_number_input() {
        // I64
        let value = json!(31);
        let result = deserializer_v1(&value).expect("Failed to deserialize an Int64");
        assert_eq!(result, GValue::Int64(31));

        // F64
        let value = json!(31.3);
        let result = deserializer_v1(&value).expect("Failed to deserialize Double");
        assert_eq!(result, GValue::Double(31.3));
    }
}
