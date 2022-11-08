use pa_rs::*;

#[derive(Debug, PartialEq)]
pub enum JSONValue {
    JSONNull,
    JSONBool(bool),
    JSONNumber(f64),
    JSONString(String),
    JSONArray(Vec<JSONValue>),
    JSONObject(Vec<(String, JSONValue)>),
}

use JSONValue::*;

pub fn parse_json() -> Parser<JSONValue> {
    parse_one_of([
        parse_json_object(),
        parse_json_array(),
        parse_json_string(),
        parse_json_number(),
        parse_json_bool(),
        parse_json_null(),
    ])
}

fn parse_json_null() -> Parser<JSONValue> {
    parse_str("null").map(|_| JSONNull)
}

fn parse_json_bool() -> Parser<JSONValue> {
    parse_bool().map(|x| JSONBool(x))
}

fn parse_json_number() -> Parser<JSONValue> {
    parse_float().map(|x| JSONNumber(x))
}

fn parse_json_string() -> Parser<JSONValue> {
    parse_string().map(|x| JSONString(x))
}

fn parse_json_array() -> Parser<JSONValue> {
    Parser::new(|input| parse_list_of(|| parse_json())._run(input)).map(|v| JSONArray(v))
}

fn parse_json_object() -> Parser<JSONValue> {
    let parse_kv = || parse_sbws(parse_string()) << parse_char(':') & parse_sbws(parse_json());
    parse_char('{')
        >> parse_one_of([
            Parser::new(move |input| {
                (parse_zero_or_more(parse_kv() << parse_char(',')) & parse_kv())
                    .map(|(mut v, a)| {
                        v.push(a);
                        JSONObject(v)
                    })
                    ._run(input)
            }),
            parse_white_space().map(|_| JSONObject(vec![])),
        ])
        << parse_char('}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json() {
        let raw_json = "\
        {\
            \"squadName\": \"Super hero squad\",\
            \"homeTown\": \"Metro City\",\
            \"formed\": 2016,\
            \"secretBase\": \"Super tower\",\
            \"active\": true,\
            \"members\": [\
              {\
                \"name\": \"Molecule Man\",\
                \"age\": 29,\
                \"secretIdentity\": \"Dan Jukes\",\
                \"powers\": [\"Radiation resistance\", \"Turning tiny\", \"Radiation blast\"]\
              },\
              {\
                \"name\": \"Madame Uppercut\",\
                \"age\": 39,\
                \"secretIdentity\": \"Jane Wilson\",\
                \"powers\": [\
                  \"Million tonne punch\",\
                  \"Damage resistance\",\
                  \"Superhuman reflexes\"\
                ]\
              },\
              {\
                \"name\": \"Eternal Flame\",\
                \"age\": 1000000,\
                \"secretIdentity\": \"Unknown\",\
                \"powers\": [\
                  \"Immortality\",\
                  \"Heat Immunity\",\
                  \"Inferno\",\
                  \"Teleportation\",\
                  \"Interdimensional travel\"\
                ]\
              }\
            ]\
          }\
        ";

        let s = String::from;

        let parsed_json = JSONObject(vec![
            (s("squadName"), JSONString(s("Super hero squad"))),
            (s("homeTown"), JSONString(s("Metro City"))),
            (s("formed"), JSONNumber(2016.0)),
            (s("secretBase"), JSONString(s("Super tower"))),
            (s("active"), JSONBool(true)),
            (
                s("members"),
                JSONArray(vec![
                    JSONObject(vec![
                        (s("name"), JSONString(s("Molecule Man"))),
                        (s("age"), JSONNumber(29.0)),
                        (s("secretIdentity"), JSONString(s("Dan Jukes"))),
                        (
                            s("powers"),
                            JSONArray(vec![
                                JSONString(s("Radiation resistance")),
                                JSONString(s("Turning tiny")),
                                JSONString(s("Radiation blast")),
                            ]),
                        ),
                    ]),
                    JSONObject(vec![
                        (s("name"), JSONString(s("Madame Uppercut"))),
                        (s("age"), JSONNumber(39.0)),
                        (s("secretIdentity"), JSONString(s("Jane Wilson"))),
                        (
                            s("powers"),
                            JSONArray(vec![
                                JSONString(s("Million tonne punch")),
                                JSONString(s("Damage resistance")),
                                JSONString(s("Superhuman reflexes")),
                            ]),
                        ),
                    ]),
                    JSONObject(vec![
                        (s("name"), JSONString(s("Eternal Flame"))),
                        (s("age"), JSONNumber(1000000.0)),
                        (s("secretIdentity"), JSONString(s("Unknown"))),
                        (
                            s("powers"),
                            JSONArray(vec![
                                JSONString(s("Immortality")),
                                JSONString(s("Heat Immunity")),
                                JSONString(s("Inferno")),
                                JSONString(s("Teleportation")),
                                JSONString(s("Interdimensional travel")),
                            ]),
                        ),
                    ]),
                ]),
            ),
        ]);

        assert_eq!(parse_json().run(raw_json), Ok(parsed_json));
    }
}
