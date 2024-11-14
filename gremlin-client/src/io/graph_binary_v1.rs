use std::{collections::HashMap, convert::TryInto};

use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

use crate::{
    conversion::FromGValue,
    io::graph_binary_v1,
    message::{ReponseStatus, Response, ResponseResult},
    process::traversal::{Instruction, Order, Scope},
    structure::{Column, Direction, Merge, Pop, TextP, Traverser, T},
    Cardinality, Edge, GKey, GValue, GremlinError, GremlinResult, Metric, Path, Property, ToGValue,
    TraversalMetrics, Vertex, VertexProperty, GID,
};

use super::IoProtocol;

const VERSION_BYTE: u8 = 0x81;
const VALUE_FLAG: u8 = 0x00;
const VALUE_NULL_FLAG: u8 = 0x01;

//Data codes (https://tinkerpop.apache.org/docs/current/dev/io/#_data_type_codes)
const INTEGER: u8 = 0x01;
const LONG: u8 = 0x02;
const STRING: u8 = 0x03;
const DATE: u8 = 0x04;
// const TIMESTAMP: u8 = 0x05;
// const CLASS: u8 = 0x06;
const DOUBLE: u8 = 0x07;
const FLOAT: u8 = 0x08;
const LIST: u8 = 0x09;
const MAP: u8 = 0x0A;
const SET: u8 = 0x0B;
const UUID: u8 = 0x0C;
const EDGE: u8 = 0x0D;
const PATH: u8 = 0x0E;
const PROPERTY: u8 = 0x0F;
// const TINKERGRAPH: u8 = 0x10;
const VERTEX: u8 = 0x11;
const VERTEX_PROPERTY: u8 = 0x12;
// const BARRIER: u8 = 0x13;
// const BINDING: u8 = 0x14;
const BYTECODE: u8 = 0x15;
const CARDINALITY: u8 = 0x16;
const COLUMN: u8 = 0x17;
const DIRECTION: u8 = 0x18;
// const OPERATOR: u8 = 0x19;
const ORDER: u8 = 0x1A;
// const PICK: u8 = 0x1B;
const POP: u8 = 0x1C;
// const LAMBDA: u8 = 0x1D;
const P: u8 = 0x1E;
const SCOPE: u8 = 0x1F;
const T: u8 = 0x20;
const TRAVERSER: u8 = 0x21;
const BOOLEAN: u8 = 0x27;
const TEXTP: u8 = 0x28;
const MERTRICS: u8 = 0x2C;
const TRAVERSAL_MERTRICS: u8 = 0x2D;
const MERGE: u8 = 0x2E;
const UNSPECIFIED_NULL_OBEJECT: u8 = 0xFE;
const CUSTOM: u8 = 0x00;

pub(crate) struct RequestMessage<'a, 'b> {
    pub(crate) request_id: Uuid,
    pub(crate) op: &'a str,
    pub(crate) processor: &'b str,
    pub(crate) args: HashMap<String, GValue>,
}

pub(crate) struct ResponseMessage {
    //Format: {version}{request_id}{status_code}{status_message}{status_attributes}{result_meta}{result_data}
    pub(crate) request_id: Option<Uuid>,
    pub(crate) status_code: i16,
    pub(crate) status_message: String,
    pub(crate) status_attributes: HashMap<GKey, GValue>,
    pub(crate) result_meta: HashMap<GKey, GValue>,
    pub(crate) result_data: Option<GValue>,
}

impl Into<Response> for ResponseMessage {
    fn into(self) -> Response {
        let status = ReponseStatus {
            code: self.status_code,
            message: self.status_message,
        };
        Response {
            request_id: self.request_id,
            result: ResponseResult {
                data: self.result_data,
            },
            status,
        }
    }
}

impl GraphBinaryV1Deser for HashMap<GKey, GValue> {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        //first will be the map length
        let map_length = <i32 as GraphBinaryV1Deser>::from_be_bytes(bytes)?;
        let mut map = HashMap::new();
        //Then fully qualified entry of each k/v pair
        for _ in 0..map_length {
            let key: GKey = GKey::from_gvalue(GValue::from_be_bytes(bytes)?)
                .map_err(|_| GremlinError::Cast(format!("Invalid GKey bytes")))?;
            let value = GValue::from_be_bytes(bytes)?;

            map.insert(key, value);
        }
        Ok(map)
    }
}

impl GraphBinaryV1Deser for ResponseMessage {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        //First confirm the version is as expected
        let Some(&graph_binary_v1::VERSION_BYTE) = bytes.next() else {
            return Err(GremlinError::Cast(format!("Invalid version byte")));
        };

        //Request id is nullable
        let request_id = Uuid::from_be_bytes_nullable(bytes)?;

        let status_code = <i32 as GraphBinaryV1Deser>::from_be_bytes(bytes)?
            .try_into()
            .expect("Status code should fit in i16");
        //Status message is nullable
        let status_message = String::from_be_bytes_nullable(bytes)?.unwrap_or_default();

        let status_attributes = GraphBinaryV1Deser::from_be_bytes(bytes)?;
        let result_meta: HashMap<GKey, GValue> = GraphBinaryV1Deser::from_be_bytes(bytes)?;
        let result_data = GValue::from_be_bytes(bytes)?;
        let result_data = if result_data == GValue::Null {
            None
        } else {
            Some(result_data)
        };
        Ok(ResponseMessage {
            request_id,
            status_code,
            status_message,
            status_attributes,
            result_meta,
            result_data,
        })
    }
}

impl<'a, 'b> GraphBinaryV1Ser for RequestMessage<'a, 'b> {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        //Need to write header first, its length is a Byte not a Int
        let header = IoProtocol::GraphBinaryV1.content_type();
        let header_length: u8 = header
            .len()
            .try_into()
            .expect("Header length should fit in u8");
        buf.push(header_length);
        buf.extend_from_slice(header.as_bytes());

        //Version byte first
        buf.push(VERSION_BYTE);

        //Request Id
        self.request_id.to_be_bytes(buf)?;

        //Op
        self.op.to_be_bytes(buf)?;

        //Processor
        self.processor.to_be_bytes(buf)?;

        //Args
        let args_length: i32 = self
            .args
            .len()
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("Args exceeds i32 length limit")))?;
        GraphBinaryV1Ser::to_be_bytes(args_length, buf)?;
        for (k, v) in self.args.into_iter() {
            //Both keys and values need to be fully qualified here, so turn
            //the keys into a GValue
            GValue::from(k).to_be_bytes(buf)?;
            v.to_be_bytes(buf)?;
        }
        Ok(())
    }
}

//https://tinkerpop.apache.org/docs/current/dev/io/#_data_type_codes
//Each type has a "fully qualified" serialized form usually:  {type_code}{type_info}{value_flag}{value}
//{type_code} is a single unsigned byte representing the type number.
//{type_info} is an optional sequence of bytes providing additional information of the type represented. This is specially useful for representing complex and custom types.
//{value_flag} is a single byte providing information about the value. Flags have the following meaning:
// 0x01 The value is null. When this flag is set, no bytes for {value} will be provided.
//{value} is a sequence of bytes which content is determined by the type.
//All encodings are big-endian.

//However there are occassion when just "the value" is written without the fully qualified form, for example the 4 bytes of a integer without the type_code
//this is usually done in scenarios when the type in unambiguous by schema.

//Generally this is written such that serializing a value wrapped by GValue is taken to mean to write the fully qualified representation
//and serializing just "the value" is done directly upon the underlying value type

fn write_usize_as_i32_be_bytes(val: usize, buf: &mut Vec<u8>) -> GremlinResult<()> {
    let val_i32 = TryInto::<i32>::try_into(val)
        .map_err(|_| GremlinError::Cast(format!("Invalid usize bytes exceed i32")))?;
    GraphBinaryV1Ser::to_be_bytes(val_i32, buf)
}

impl GraphBinaryV1Ser for &GValue {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        match self {
            GValue::Int32(value) => {
                //Type code of 0x01
                buf.push(INTEGER);
                //Empty value flag
                buf.push(VALUE_FLAG);
                //then value bytes
                GraphBinaryV1Ser::to_be_bytes(*value, buf)?;
            }
            GValue::Int64(value) => {
                buf.push(LONG);
                buf.push(VALUE_FLAG);
                GraphBinaryV1Ser::to_be_bytes(*value, buf)?;
            }
            GValue::String(value) => {
                write_fully_qualified_str(value, buf)?;
            }
            GValue::Date(value) => {
                buf.push(DATE);
                buf.push(VALUE_FLAG);
                value.to_be_bytes(buf)?;
            }
            GValue::Double(value) => {
                buf.push(DOUBLE);
                buf.push(VALUE_FLAG);
                GraphBinaryV1Ser::to_be_bytes(*value, buf)?;
            }
            GValue::Float(value) => {
                buf.push(FLOAT);
                buf.push(VALUE_FLAG);
                GraphBinaryV1Ser::to_be_bytes(*value, buf)?;
            }
            GValue::List(value) => {
                buf.push(LIST);
                buf.push(VALUE_FLAG);

                //{length} is an Int describing the length of the collection.
                write_usize_as_i32_be_bytes(value.len(), buf)?;

                //{item_0}…​{item_n} are the items of the list. {item_i} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value}.
                for item in value.iter() {
                    item.to_be_bytes(buf)?;
                }
            }
            GValue::Map(map) => {
                //Type code of 0x0a: Map
                buf.push(MAP);
                // //Empty value flag
                buf.push(VALUE_FLAG);

                //{length} is an Int describing the length of the map.
                write_usize_as_i32_be_bytes(map.len(), buf)?;

                //{item_0}…​{item_n} are the items of the map. {item_i} is sequence of 2 fully qualified typed values one representing the key
                //  and the following representing the value, each composed of {type_code}{type_info}{value_flag}{value}.
                for (k, v) in map.iter() {
                    k.to_be_bytes(buf)?;
                    v.to_be_bytes(buf)?;
                }
            }
            GValue::Set(value) => {
                buf.push(SET);
                buf.push(VALUE_FLAG);

                //{length} is an Int describing the length of the collection.
                write_usize_as_i32_be_bytes(value.len(), buf)?;

                //{item_0}…​{item_n} are the items of the list. {item_i} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value}.
                for item in value.iter() {
                    item.to_be_bytes(buf)?;
                }
            }
            GValue::Uuid(value) => {
                buf.push(UUID);
                buf.push(VALUE_FLAG);
                value.to_be_bytes(buf)?;
            }
            GValue::Vertex(vertex) => {
                buf.push(VERTEX);
                buf.push(VALUE_FLAG);
                vertex.id().to_be_bytes(buf)?;
                vertex.label().to_be_bytes(buf)?;
                GValue::Null.to_be_bytes(buf)?;
            }
            GValue::Bytecode(code) => {
                //Type code of 0x15: Bytecode
                buf.push(BYTECODE);
                //Empty value flag
                buf.push(VALUE_FLAG);
                //then value bytes
                // {steps_length}{step_0}…​{step_n}{sources_length}{source_0}…​{source_n}
                //{steps_length} is an Int value describing the amount of steps.
                //{step_i} is composed of {name}{values_length}{value_0}…​{value_n}, where:
                //  {name} is a String.
                //  {values_length} is an Int describing the amount values.
                //  {value_i} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value} describing the step argument.

                fn write_instructions(
                    instructions: &Vec<Instruction>,
                    buf: &mut Vec<u8>,
                ) -> GremlinResult<()> {
                    write_usize_as_i32_be_bytes(instructions.len(), buf)?;
                    for instruction in instructions {
                        GraphBinaryV1Ser::to_be_bytes(instruction.operator().as_str(), buf)?;
                        write_usize_as_i32_be_bytes(instruction.args().len(), buf)?;
                        instruction
                            .args()
                            .iter()
                            .try_for_each(|arg| arg.to_be_bytes(buf))?;
                    }
                    Ok(())
                }
                write_instructions(code.steps(), buf)?;
                write_instructions(code.sources(), buf)?;
            }
            GValue::Cardinality(cardinality) => {
                buf.push(CARDINALITY);
                buf.push(VALUE_FLAG);
                cardinality.to_be_bytes(buf)?;
            }
            GValue::Column(column) => {
                buf.push(COLUMN);
                buf.push(VALUE_FLAG);
                column.to_be_bytes(buf)?;
            }
            GValue::Direction(direction) => {
                buf.push(DIRECTION);
                buf.push(VALUE_FLAG);
                direction.to_be_bytes(buf)?;
            }
            GValue::Order(order) => {
                buf.push(ORDER);
                buf.push(VALUE_FLAG);
                order.to_be_bytes(buf)?;
            }
            GValue::Pop(pop) => {
                buf.push(POP);
                buf.push(VALUE_FLAG);
                pop.to_be_bytes(buf)?;
            }
            GValue::P(p) => {
                //Type code of 0x1e: P
                buf.push(P);
                buf.push(VALUE_FLAG);
                p.to_be_bytes(buf)?;
            }
            GValue::Scope(scope) => {
                //Type code of 0x1f: Scope
                buf.push(SCOPE);
                //Empty value flag
                buf.push(VALUE_FLAG);

                scope.to_be_bytes(buf)?;
            }
            GValue::T(t) => {
                buf.push(T);
                buf.push(VALUE_FLAG);
                t.to_be_bytes(buf)?;
            }
            GValue::Bool(bool) => {
                buf.push(BOOLEAN);
                buf.push(VALUE_FLAG);
                bool.to_be_bytes(buf)?;
            }
            GValue::TextP(text_p) => {
                buf.push(TEXTP);
                buf.push(VALUE_FLAG);
                text_p.to_be_bytes(buf)?;
            }
            GValue::Merge(merge) => {
                buf.push(MERGE);
                buf.push(VALUE_FLAG);
                merge.to_be_bytes(buf)?;
            }
            GValue::Null => {
                //Type code of 0xfe: Unspecified null object
                buf.push(UNSPECIFIED_NULL_OBEJECT);
                //Then the null {value_flag} set and no sequence of bytes.
                buf.push(VALUE_NULL_FLAG);
            }
            other => unimplemented!("Serializing GValue {other:?}"),
        }
        Ok(())
    }
}

fn predicate_to_be_bytes(
    operator: &String,
    value: &GValue,
    buf: &mut Vec<u8>,
) -> GremlinResult<()> {
    operator.to_be_bytes(buf)?;
    match value {
        //Singular values have a length of 1
        //But still need to be written fully qualified
        scalar @ GValue::Uuid(_)
        | scalar @ GValue::Int32(_)
        | scalar @ GValue::Int64(_)
        | scalar @ GValue::Float(_)
        | scalar @ GValue::Double(_)
        | scalar @ GValue::String(_)
        | scalar @ GValue::Date(_) => {
            GraphBinaryV1Ser::to_be_bytes(1i32, buf)?;
            scalar.to_be_bytes(buf)?;
        }
        //"Collections" need to be unfurled, we don't write the collection but
        //instead just its lengths and then the fully qualified form of each element
        GValue::List(list) => {
            write_usize_as_i32_be_bytes(list.len(), buf)?;
            for item in list.iter() {
                item.to_be_bytes(buf)?;
            }
        }
        GValue::Set(set) => {
            write_usize_as_i32_be_bytes(set.len(), buf)?;
            for item in set.iter() {
                item.to_be_bytes(buf)?;
            }
        }
        other => unimplemented!("Predicate serialization of {other:?} not implemented"),
    }
    Ok(())
}

impl GraphBinaryV1Ser for &crate::structure::P {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        predicate_to_be_bytes(&self.operator, &self.value, buf)
    }
}

fn write_fully_qualified_str(value: &str, buf: &mut Vec<u8>) -> GremlinResult<()> {
    //Extracted from the GValue implementation so it can be used by enum string values
    //There are times we want the fully qualified prefix bytes outside of the normal
    //GValue based route, without having to allocate the string again inside the GValue

    //Type code of 0x03: String
    buf.push(STRING);
    //Empty value flag
    buf.push(VALUE_FLAG);
    value.to_be_bytes(buf)
}

impl GraphBinaryV1Ser for &Merge {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        let literal = match self {
            Merge::OnCreate => "onCreate",
            Merge::OnMatch => "onMatch",
            Merge::OutV => "outV",
            Merge::InV => "inV",
        };
        write_fully_qualified_str(literal, buf)
    }
}

impl GraphBinaryV1Ser for &Scope {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        //Format: a fully qualified single String representing the enum value.
        let literal = match self {
            Scope::Global => "global",
            Scope::Local => "local",
        };
        write_fully_qualified_str(literal, buf)
    }
}

impl GraphBinaryV1Ser for &Cardinality {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        let literal = match self {
            Cardinality::List => "list",
            Cardinality::Set => "set",
            Cardinality::Single => "single",
        };
        write_fully_qualified_str(literal, buf)
    }
}

impl GraphBinaryV1Deser for Direction {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        match GValue::from_be_bytes(bytes)? {
            GValue::String(literal) if literal.eq_ignore_ascii_case("out") => Ok(Direction::Out),
            GValue::String(literal) if literal.eq_ignore_ascii_case("in") => Ok(Direction::In),
            other => Err(GremlinError::Cast(format!(
                "Unexpected direction literal {other:?}"
            ))),
        }
    }
}

impl GraphBinaryV1Ser for &Direction {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        let literal = match self {
            Direction::Out | Direction::From => "OUT",
            Direction::In | Direction::To => "IN",
        };
        write_fully_qualified_str(literal, buf)
    }
}

impl GraphBinaryV1Ser for &Order {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        let literal = match self {
            Order::Asc => "asc",
            Order::Desc => "desc",
            Order::Shuffle => "shuffle",
        };
        write_fully_qualified_str(literal, buf)
    }
}

impl GraphBinaryV1Ser for &Pop {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        //Format: a fully qualified single String representing the enum value.
        let literal = match self {
            Pop::All => "all",
            Pop::First => "first",
            Pop::Last => "last",
            Pop::Mixed => "mixed",
        };
        write_fully_qualified_str(literal, buf)
    }
}

impl GraphBinaryV1Ser for &Column {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        //Format: a fully qualified single String representing the enum value.
        let literal = match self {
            Column::Keys => "keys",
            Column::Values => "values",
        };
        write_fully_qualified_str(literal, buf)
    }
}

impl GraphBinaryV1Ser for &GKey {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        match self {
            GKey::T(t) => GValue::T(t.clone()).to_be_bytes(buf),
            GKey::String(str) => write_fully_qualified_str(str, buf),
            GKey::Direction(direction) => GValue::Direction(direction.clone()).to_be_bytes(buf),
            GKey::Int64(i) => GValue::Int64(*i).to_be_bytes(buf),
            GKey::Int32(i) => GValue::Int32(*i).to_be_bytes(buf),
            other => unimplemented!("Unimplemented GKey serialization requested {other:?}"),
        }
    }
}
pub trait GraphBinaryV1Ser: Sized {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()>;
}

pub trait GraphBinaryV1Deser: Sized {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self>;

    fn from_be_bytes_nullable<'a, S: Iterator<Item = &'a u8>>(
        bytes: &mut S,
    ) -> GremlinResult<Option<Self>> {
        match bytes.next().cloned() {
            Some(VALUE_FLAG) => Self::from_be_bytes(bytes).map(Option::Some),
            Some(VALUE_NULL_FLAG) => Ok(None),
            other => {
                return Err(GremlinError::Cast(format!(
                    "Unexpected byte for nullable check: {other:?}"
                )));
            }
        }
    }
}

impl GraphBinaryV1Ser for bool {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        //Format: A single byte containing the value 0x01 when it’s true and 0 otherwise.
        match self {
            true => buf.push(0x01),
            false => buf.push(0x00),
        }
        Ok(())
    }
}

impl GraphBinaryV1Ser for &TextP {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        predicate_to_be_bytes(&self.operator, &self.value, buf)
    }
}

impl GraphBinaryV1Deser for bool {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        match bytes.next() {
            Some(0x00) => Ok(false),
            Some(0x01) => Ok(true),
            other => Err(GremlinError::Cast(format!(
                "No boolean value byte {other:?}"
            ))),
        }
    }
}

impl GraphBinaryV1Deser for GValue {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        let data_code = bytes
            .next()
            .ok_or_else(|| GremlinError::Cast(format!("Invalid bytes no data code byte")))?;
        match *data_code {
            INTEGER => Ok(match i32::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::Int32(value),
                None => GValue::Null,
            }),
            LONG => Ok(match i64::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::Int64(value),
                None => GValue::Null,
            }),
            STRING => Ok(match String::from_be_bytes_nullable(bytes)? {
                Some(string) => GValue::String(string),
                None => GValue::Null,
            }),
            DATE => match i64::from_be_bytes_nullable(bytes)? {
                Some(value) => match Utc.timestamp_millis_opt(value) {
                    chrono::LocalResult::Single(valid) => Ok(GValue::Date(valid)),
                    _ => Err(GremlinError::Cast(format!("Invalid timestamp millis"))),
                },
                None => Ok(GValue::Null),
            },
            DOUBLE => Ok(match f64::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::Double(value),
                None => GValue::Null,
            }),
            FLOAT => Ok(match f32::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::Float(value),
                None => GValue::Null,
            }),
            LIST => {
                let deserialized_list: Option<Vec<GValue>> =
                    GraphBinaryV1Deser::from_be_bytes_nullable(bytes)?;
                Ok(deserialized_list
                    .map(|val| GValue::List(val.into()))
                    .unwrap_or(GValue::Null))
            }
            MAP => {
                let deserialized_map: Option<HashMap<GKey, GValue>> =
                    GraphBinaryV1Deser::from_be_bytes_nullable(bytes)?;
                Ok(deserialized_map
                    .map(|val| GValue::Map(val.into()))
                    .unwrap_or(GValue::Null))
            }
            SET => {
                let deserialized_set: Option<Vec<GValue>> =
                    GraphBinaryV1Deser::from_be_bytes_nullable(bytes)?;
                Ok(deserialized_set
                    .map(|val| GValue::Set(val.into()))
                    .unwrap_or(GValue::Null))
            }
            UUID => Ok(match Uuid::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::Uuid(value),
                None => GValue::Null,
            }),
            EDGE => Ok(match Edge::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::Edge(value),
                None => GValue::Null,
            }),
            PATH => Ok(match Path::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::Path(value),
                None => GValue::Null,
            }),
            PROPERTY => Ok(match Property::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::Property(value),
                None => GValue::Null,
            }),
            VERTEX => Ok(match Vertex::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::Vertex(value),
                None => GValue::Null,
            }),
            VERTEX_PROPERTY => Ok(match VertexProperty::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::VertexProperty(value),
                None => GValue::Null,
            }),
            DIRECTION => Ok(match Direction::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::Direction(value),
                None => GValue::Null,
            }),
            T => Ok(match T::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::T(value),
                None => GValue::Null,
            }),
            TRAVERSER => {
                let traverser: Option<Traverser> =
                    GraphBinaryV1Deser::from_be_bytes_nullable(bytes)?;
                Ok(traverser
                    .map(|val| GValue::Traverser(val))
                    .unwrap_or(GValue::Null))
            }
            BOOLEAN => Ok(match bool::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::Bool(value),
                None => GValue::Null,
            }),
            MERTRICS => Ok(match Metric::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::Metric(value),
                None => GValue::Null,
            }),
            TRAVERSAL_MERTRICS => Ok(match TraversalMetrics::from_be_bytes_nullable(bytes)? {
                Some(value) => GValue::TraversalMetrics(value),
                None => GValue::Null,
            }),
            UNSPECIFIED_NULL_OBEJECT => {
                //Need to confirm the null-ness with the next byte being a 1
                match bytes.next().cloned() {
                    Some(VALUE_NULL_FLAG) => Ok(GValue::Null),
                    other => Err(GremlinError::Cast(format!(
                        "Not expected null value byte {other:?}"
                    ))),
                }
            }
            CUSTOM => {
                let custom_name = String::from_be_bytes(bytes)?;
                match custom_name.as_str() {
                    "janusgraph.RelationIdentifier" => {
                        let deserialized: Option<JanusGraphRelationIdentifier> =
                            GraphBinaryV1Deser::from_be_bytes(bytes)?;
                        Ok(deserialized
                            .map(|value| {
                                //We don't have a GValue for JG types, and moreover supporting future custom types we may not want to
                                //so for now just mapping it to a GValue::String
                                let mut converted = format!(
                                    "{}-{}-{}",
                                    value.relation_id, value.out_vertex_id, value.type_id
                                );
                                if let Some(in_vertex_id) = value.in_vertex_id {
                                    converted.push('-');
                                    converted.push_str(&in_vertex_id);
                                }
                                GValue::String(converted)
                            })
                            .unwrap_or(GValue::Null))
                    }
                    other => unimplemented!("Unimplemented handling of custom type {other}"),
                }
            }
            other => {
                let remainder: Vec<u8> = bytes.cloned().collect();
                unimplemented!("Unimplemented deserialization byte {other}. {remainder:?}");
            }
        }
    }
}

#[derive(Debug)]
struct JanusGraphRelationIdentifier {
    out_vertex_id: String,
    type_id: i64,
    relation_id: i64,
    in_vertex_id: Option<String>,
}

impl GraphBinaryV1Deser for Option<JanusGraphRelationIdentifier> {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        //Confirm the marker bytes that should be next
        //0x1001
        let marker_bytes: i32 = GraphBinaryV1Deser::from_be_bytes(bytes)?;
        if marker_bytes != 0x1001 {
            return Err(GremlinError::Cast(format!(
                "Unexpected marker bytes for JanusGraphRelationIdentifier"
            )));
        }

        match bytes.next() {
            Some(0x00) => {
                //nothing to do
            }
            Some(0x01) => return Ok(None),
            _ => return Err(GremlinError::Cast(format!("Invalid null value byte"))),
        }

        fn read_string<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<String> {
            let mut string = String::new();
            //JG custom string serialization uses the high portion of a byte to indicate the terminus of the string
            //so we'll need to mask out the value from the low half of the byte and then check the high portion
            //for termination
            loop {
                let Some(byte) = bytes.next() else {
                    return Err(GremlinError::Cast(format!(
                        "Exhausted bytes before terminal string marker"
                    )));
                };
                string.push((byte & 0x7F) as char);

                if byte & 0x80 > 0 {
                    break;
                }
            }
            Ok(string)
        }

        fn read_vertex_id<'a, S: Iterator<Item = &'a u8>>(
            bytes: &mut S,
        ) -> GremlinResult<Option<String>> {
            //There should be a type marker of either Long(0) or String(1)
            //reuse bool here, mapping false to a Long and true to String
            if bool::from_be_bytes(bytes)? {
                Ok(Some(read_string(bytes)?))
            } else {
                let value = <i64 as GraphBinaryV1Deser>::from_be_bytes(bytes)?;
                if value == 0 {
                    Ok(None)
                } else {
                    Ok(Some(value.to_string()))
                }
            }
        }

        let out_vertex_id = read_vertex_id(bytes)?.expect("Out vertex id should never be null");
        let type_id: i64 = GraphBinaryV1Deser::from_be_bytes(bytes)?;
        let relation_id: i64 = GraphBinaryV1Deser::from_be_bytes(bytes)?;
        let in_vertex_id = read_vertex_id(bytes)?;
        Ok(Some(JanusGraphRelationIdentifier {
            out_vertex_id,
            type_id,
            relation_id,
            in_vertex_id,
        }))
    }
}

impl GraphBinaryV1Deser for TraversalMetrics {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        //Format: {duration}{metrics}

        //{duration} is a Long describing the duration in nanoseconds
        let duration: i64 = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        //{metrics} is a List composed by Metrics items
        let metrics: Vec<GValue> = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        let metrics: Result<Vec<Metric>, GremlinError> = metrics
            .into_iter()
            .map(|value| Metric::from_gvalue(value))
            .collect();

        //It doesn't appear documented but assuming the duration unit inherited from GraphSON is ms, so convert here
        let duration_ms = duration as f64 / 1_000.0;
        Ok(TraversalMetrics::new(duration_ms, metrics?))
    }
}

impl GraphBinaryV1Deser for Metric {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        //Format: {id}{name}{duration}{counts}{annotations}{nested_metrics}

        //{id} is a String representing the identifier
        let id = String::from_be_bytes(bytes)?;
        //{name} is a String representing the name
        let name = String::from_be_bytes(bytes)?;

        //{duration} is a Long describing the duration in nanoseconds
        let duration: i64 = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        //{counts} is a Map composed by String keys and Long values
        let mut counts: HashMap<GKey, GValue> = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        //{annotations} is a Map composed by String keys and a value of any type
        let mut annotations: HashMap<GKey, GValue> = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        //{nested_metrics} is a List composed by Metrics items
        let nested = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        let traverser_count =
            i64::from_gvalue(counts.remove(&GKey::from("traverserCount")).ok_or(
                GremlinError::Cast(format!("Missing expected traverserCount property")),
            )?)?;

        let element_count = i64::from_gvalue(counts.remove(&GKey::from("elementCount")).ok_or(
            GremlinError::Cast(format!("Missing expected elementCount property")),
        )?)?;
        let percent_dur = f64::from_gvalue(annotations.remove(&GKey::from("percentDur")).ok_or(
            GremlinError::Cast(format!("Missing expected percentDur property")),
        )?)?;

        //It doesn't appear documented but assuming the duration unit inherited from GraphSON is ms, so convert here
        let duration_ms = duration as f64 / 1_000.0;

        Ok(Metric::new(
            id,
            name,
            duration_ms,
            element_count,
            traverser_count,
            percent_dur,
            nested,
        ))
    }
}

impl GraphBinaryV1Deser for T {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        let literal = GValue::from_be_bytes(bytes)?;
        match literal {
            GValue::String(literal) if literal.eq_ignore_ascii_case("id") => Ok(T::Id),
            GValue::String(literal) if literal.eq_ignore_ascii_case("key") => Ok(T::Key),
            GValue::String(literal) if literal.eq_ignore_ascii_case("label") => Ok(T::Label),
            GValue::String(literal) if literal.eq_ignore_ascii_case("value") => Ok(T::Value),
            other => Err(GremlinError::Cast(format!(
                "Unexpected T literal {other:?}"
            ))),
        }
    }
}

impl GraphBinaryV1Ser for &T {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        let literal = match self {
            T::Id => "id",
            T::Key => "key",
            T::Label => "label",
            T::Value => "value",
        };
        write_fully_qualified_str(literal, buf)
    }
}

fn consume_expected_null_reference_bytes<'a, S: Iterator<Item = &'a u8>>(
    bytes: &mut S,
    null_reference_descriptor: &str,
) -> GremlinResult<()> {
    let GValue::Null = GraphBinaryV1Deser::from_be_bytes(bytes)? else {
        //Anything else is erroneous
        return Err(GremlinError::Cast(format!(
            "{null_reference_descriptor} is supposed to be a \"null\" reference"
        )));
    };

    Ok(())
}

impl GraphBinaryV1Deser for Edge {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        //Format: {id}{label}{inVId}{inVLabel}{outVId}{outVLabel}{parent}{properties}

        //{id} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value}
        let id: GValue = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        //{label} is a String value.
        let label: String = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        //Ideally we'd just do something like Vertex::from_be_bytes(bytes) for the in/out vertices
        //however only the id & label is submitted, the "null" properties byte is not

        //{inVId} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value}.
        let in_v_id: GValue = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        //{inVLabel} is a String value.
        let in_v_label: String = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        //{outVId} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value}.
        let out_v_id: GValue = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        //{outVLabel} is a String value.
        let out_v_label: String = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        //{parent} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value} which contains the parent Vertex. Note that as TinkerPop currently send "references" only, this value will always be null.
        consume_expected_null_reference_bytes(bytes, "Parent")?;

        //{properties} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value} which contains the properties for the edge.
        let properties = match GValue::from_be_bytes(bytes)? {
            GValue::Null => HashMap::new(),
            GValue::List(raw_properties) => raw_properties
                .into_iter()
                .map(|property| {
                    property
                        .take::<Property>()
                        .map(|converted| (converted.label().clone(), converted))
                })
                .collect::<GremlinResult<_>>()?,
            _ => {
                return Err(GremlinError::Cast(format!(
                    "Edge properties should either be Null or List"
                )))
            }
        };
        Ok(Edge::new(
            id.try_into()?,
            label,
            in_v_id.try_into()?,
            in_v_label,
            out_v_id.try_into()?,
            out_v_label,
            properties,
        ))
    }
}

impl GraphBinaryV1Deser for Property {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        //Format: {key}{value}{parent}

        //{key} is a String value
        let key: String = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        //{value} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value}
        let value = GValue::from_be_bytes(bytes)?;

        //{parent} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value} which is either an Edge or VertexProperty.
        //Note that as TinkerPop currently sends "references" only this value will always be null.
        consume_expected_null_reference_bytes(bytes, "Parent")?;
        Ok(Property::new(key, value))
    }
}

impl GraphBinaryV1Deser for Path {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        let labels = GValue::from_be_bytes(bytes)?;
        let GValue::List(objects) = GValue::from_be_bytes(bytes)? else {
            return Err(GremlinError::Cast(format!("Path objects should be a list")));
        };
        Ok(Path::new(labels, objects))
    }
}

impl GraphBinaryV1Deser for VertexProperty {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        //Format: {id}{label}{value}{parent}{properties}
        //{id} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value}.
        let id: GValue = GValue::from_be_bytes(bytes)?;
        let id: GID = id.try_into()?;
        //{label} is a String value.
        let label: String = GraphBinaryV1Deser::from_be_bytes(bytes)?;

        //{value} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value}.
        let value: GValue = GValue::from_be_bytes(bytes)?;

        //{parent} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value} which contains the parent Vertex.
        //Note that as TinkerPop currently send "references" only, this value will always be null.
        consume_expected_null_reference_bytes(bytes, "Parent vertex")?;

        //{properties} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value} which contains properties.
        let _ = GValue::from_be_bytes(bytes)?; //we don't have a place for this?
        Ok(VertexProperty::new(id, label, value))
    }
}

impl GraphBinaryV1Deser for Vertex {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        //Format: {id}{label}{properties}
        //{id} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value}.
        let id: GValue = GraphBinaryV1Deser::from_be_bytes(bytes)?;
        //{label} is a String value.
        let label: String = GraphBinaryV1Deser::from_be_bytes(bytes)?;
        //{properties} is a fully qualified typed value composed of {type_code}{type_info}{value_flag}{value} which contains properties.
        //properties: HashMap<String, Vec<VertexProperty>>,
        let properties: GValue = GraphBinaryV1Deser::from_be_bytes(bytes)?;
        match properties {
            GValue::Null => Ok(Vertex::new(id.try_into()?, label, HashMap::new())),
            GValue::List(list) => {
                let mut properties = HashMap::new();
                for element in list.into_iter() {
                    let vp: VertexProperty = element.take()?;
                    properties.insert(vp.label().clone(), vec![vp]);
                }
                Ok(Vertex::new(id.try_into()?, label, properties))
            }
            other => Err(GremlinError::Cast(format!(
                "Unsupported vertex property type: {other:?}"
            ))),
        }
    }
}

impl GraphBinaryV1Ser for &str {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        //Format: {length}{text_value}
        // {length} is an Int describing the byte length of the text. Length is a positive number or zero to represent the empty string.
        // {text_value} is a sequence of bytes representing the string value in UTF8 encoding.
        let length: i32 = self
            .len()
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("String length exceeds i32")))?;
        GraphBinaryV1Ser::to_be_bytes(length, buf)?;
        buf.extend_from_slice(self.as_bytes());
        Ok(())
    }
}

impl GraphBinaryV1Ser for &DateTime<Utc> {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        //Format: An 8-byte two’s complement signed integer representing a millisecond-precision offset from the unix epoch.
        GraphBinaryV1Ser::to_be_bytes(self.timestamp_millis(), buf)
    }
}

impl GraphBinaryV1Ser for f32 {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        buf.extend_from_slice(&self.to_be_bytes());
        Ok(())
    }
}

impl GraphBinaryV1Ser for f64 {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        buf.extend_from_slice(&self.to_be_bytes());
        Ok(())
    }
}

impl GraphBinaryV1Deser for Traverser {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        Ok(Traverser::new(
            GraphBinaryV1Deser::from_be_bytes(bytes)?,
            GraphBinaryV1Deser::from_be_bytes(bytes)?,
        ))
    }
}

impl<T: GraphBinaryV1Deser> GraphBinaryV1Deser for Vec<T> {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        let length = <i32 as GraphBinaryV1Deser>::from_be_bytes(bytes)?
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("list length exceeds usize")))?;
        let mut list = Vec::new();
        list.reserve_exact(length);
        for _ in 0..length {
            list.push(T::from_be_bytes(bytes)?);
        }
        Ok(list)
    }
}

impl GraphBinaryV1Deser for String {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        let string_bytes_length: i32 = GraphBinaryV1Deser::from_be_bytes(bytes)
            .map_err(|_| GremlinError::Cast(format!("Invalid bytes for String length")))?;
        let string_bytes_length = string_bytes_length
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("String length did not fit into usize")))?;
        let string_value_bytes: Vec<u8> = bytes.take(string_bytes_length).cloned().collect();
        if string_value_bytes.len() < string_bytes_length {
            return Err(GremlinError::Cast(format!(
                "Missing bytes for String value. Expected {} only retrieved {}",
                string_bytes_length,
                string_value_bytes.len(),
            )));
        }
        String::from_utf8(string_value_bytes)
            .map_err(|_| GremlinError::Cast(format!("Invalid bytes for String value")))
    }
}

impl GraphBinaryV1Ser for i32 {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        //Format: 4-byte two’s complement integer
        buf.extend_from_slice(&self.to_be_bytes());
        Ok(())
    }
}

impl GraphBinaryV1Ser for i64 {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        //Format: 8-byte two’s complement integer
        buf.extend_from_slice(&self.to_be_bytes());
        Ok(())
    }
}

impl GraphBinaryV1Deser for i32 {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        bytes
            .take(4)
            .cloned()
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("Invalid bytes into i32")))
            .map(i32::from_be_bytes)
    }
}

impl GraphBinaryV1Deser for i64 {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        //Format: 8-byte two’s complement integer
        bytes
            .take(8)
            .cloned()
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("Invalid bytes into i64")))
            .map(i64::from_be_bytes)
    }
}

impl GraphBinaryV1Deser for f64 {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        bytes
            .take(8)
            .cloned()
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("Invalid bytes into f64")))
            .map(f64::from_be_bytes)
    }
}

impl GraphBinaryV1Deser for f32 {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        bytes
            .take(4)
            .cloned()
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("Invalid bytes into f32")))
            .map(f32::from_be_bytes)
    }
}

impl GraphBinaryV1Ser for &Uuid {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        buf.extend_from_slice(self.as_bytes().as_slice());
        Ok(())
    }
}

impl GraphBinaryV1Ser for &GID {
    fn to_be_bytes(self, buf: &mut Vec<u8>) -> GremlinResult<()> {
        match self {
            GID::String(s) => s.to_gvalue().to_be_bytes(buf),
            GID::Int32(i) => i.to_gvalue().to_be_bytes(buf),
            GID::Int64(i) => i.to_gvalue().to_be_bytes(buf),
        }
    }
}

impl GraphBinaryV1Deser for Uuid {
    fn from_be_bytes<'a, S: Iterator<Item = &'a u8>>(bytes: &mut S) -> GremlinResult<Self> {
        bytes
            .take(16)
            .cloned()
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| GremlinError::Cast(format!("Invalid bytes into Uuid")))
            .map(Uuid::from_bytes)
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use rstest::rstest;
    use std::iter;
    use uuid::uuid;

    use super::*;

    #[rstest]
    //Non-Null i32 Integer (01 00)
    #[case::int_1(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x01], GValue::Int32(1))]
    #[case::int_256(&[0x01, 0x00, 0x00, 0x00, 0x01, 0x00], GValue::Int32(256))]
    #[case::int_257(&[0x01, 0x00, 0x00, 0x00, 0x01, 0x01], GValue::Int32(257))]
    #[case::int_neg_1(&[0x01, 0x00, 0xFF, 0xFF, 0xFF, 0xFF], GValue::Int32(-1))]
    #[case::int_neg_2(&[0x01, 0x00, 0xFF, 0xFF, 0xFF, 0xFE], GValue::Int32(-2))]
    //Non-Null i64 Long (02 00)
    #[case::long_1(&[0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01], GValue::Int64(1))]
    #[case::long_neg_2(&[0x02, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE], GValue::Int64(-2))]
    //Non-Null Strings (03 00)
    #[case::str_abc(&[0x03, 0x00, 0x00, 0x00, 0x00, 0x03, 0x61, 0x62, 0x63], GValue::String("abc".into()))]
    #[case::str_abcd(&[0x03, 0x00, 0x00, 0x00, 0x00, 0x04, 0x61, 0x62, 0x63, 0x64], GValue::String("abcd".into()))]
    #[case::empty_str(&[0x03, 0x00, 0x00, 0x00, 0x00, 0x00], GValue::String("".into()))]
    //Non-Null Date (04 00)
    #[case::date_epoch(&[0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], GValue::Date(DateTime::parse_from_rfc3339("1970-01-01T00:00:00.000Z").unwrap().into()))]
    #[case::date_before_epoch(&[0x04, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], GValue::Date(DateTime::parse_from_rfc3339("1969-12-31T23:59:59.999Z").unwrap().into()))]
    //Non-Null Timestamp (05 00), no GValue at this time
    //Non-Null Class (06 00), no GValue at this time
    //Non-Null Double (07 00)
    #[case::double_1(&[0x07, 0x00, 0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], GValue::Double(1f64))]
    #[case::double_fractional(&[0x07, 0x00, 0x3F, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], GValue::Double(0.00390625))]
    #[case::double_0_dot_1(&[0x07, 0x00, 0x3F, 0xB9, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9A], GValue::Double(0.1))]
    //Non-Null Float (08 00)
    #[case::double_fractional(&[0x08, 0x00, 0x3F, 0x80, 0x00, 0x00], GValue::Float(1f32))]
    #[case::double_0_dot_1(&[0x08, 0x00, 0x3E, 0xC0, 0x00, 0x00], GValue::Float(0.375f32))]
    //Non-Null List (09 00)
    #[case::list_single_int(&[0x09, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01], GValue::List(Vec::from([GValue::Int32(1)]).into()))]
    //Non-Null Map (0A 00)
    #[case::map_single_str_int_pair(&[0x0A, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00, 0x03, 0x61, 0x62, 0x63, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01], GValue::Map(iter::once((GKey::String("abc".into()), GValue::Int32(1))).collect::<HashMap<GKey, GValue>>().into()))]
    //Non-Null Set (0B 00)
    #[case::set_single_int(&[0x0B, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01], GValue::Set(Vec::from([GValue::Int32(1)]).into()))]
    //Non-Null UUID (0C 00)
    #[case::uuid(&[0x0C, 0x00, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF], GValue::Uuid(uuid!("00112233-4455-6677-8899-aabbccddeeff")))]
    fn serde_values(#[case] expected_serialized: &[u8], #[case] expected: GValue) {
        let mut serialized = Vec::new();
        (&expected)
            .to_be_bytes(&mut serialized)
            .expect("Shouldn't fail parsing");
        assert_eq!(serialized, expected_serialized);
        let deserialized: GValue = GraphBinaryV1Deser::from_be_bytes(&mut serialized.iter())
            .expect("Shouldn't fail parsing");
        assert_eq!(deserialized, expected);
    }

    #[rstest]
    #[case::too_few_bytes( &[0x01, 0x00, 0x00, 0x00, 0x00])]
    fn serde_int32_invalid_bytes(#[case] bytes: &[u8]) {
        <GValue as GraphBinaryV1Deser>::from_be_bytes(&mut bytes.iter())
            .expect_err("Should have failed due invalid bytes");
    }
}
