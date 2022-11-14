
#[derive(Debug, PartialEq, Clone)]
pub enum JSONValue {
    JSONNull,
    JSONBool(bool),
    JSONNumber(f64),
    JSONString(String),
    JSONArray(Vec<JSONValue>),
    JSONObject(Vec<(String, JSONValue)>),
}