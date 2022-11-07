use pa_rs::*;

#[derive(Debug, PartialEq)]
pub enum JSONValue {
    JSONNull,
    JSONBool(bool),
    JSONInteger(i64),
    JSONFloat(f64),
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
        parse_json_integer(),
        parse_json_float(),
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

fn parse_json_integer() -> Parser<JSONValue> {
    parse_int().map(|x| JSONInteger(x))
}

fn parse_json_float() -> Parser<JSONValue> {
    parse_float().map(|x| JSONFloat(x))
}

fn parse_json_string() -> Parser<JSONValue> {
    parse_string().map(|x| JSONString(x))
}

fn ws<T: 'static>(p: Parser<T>) -> Parser<T> {
    parse_white_space() >> p << parse_white_space()
}

fn parse_json_array() -> Parser<JSONValue> {
    parse_char('[')
        >> (parse_one_of([
            Parser::new(|input| {
                (parse_zero_or_more(ws(parse_json()) << parse_char(',') << parse_white_space())
                    & ws(parse_json()))
                .map(|(mut v, a)| {
                    v.push(a);
                    JSONArray(v)
                })
                ._run(input)
            }),
            parse_white_space().map(|_| JSONArray(vec![])),
        ]))
        << parse_char(']')
}

fn parse_json_object() -> Parser<JSONValue> {
    let parse_kv = || ws(parse_string()) << parse_char(':') & ws(parse_json());
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
            (s("formed"), JSONInteger(2016)),
            (s("secretBase"), JSONString(s("Super tower"))),
            (s("active"), JSONBool(true)),
            (
                s("members"),
                JSONArray(vec![
                    JSONObject(vec![
                        (s("name"), JSONString(s("Molecule Man"))),
                        (s("age"), JSONInteger(29)),
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
                        (s("age"), JSONInteger(39)),
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
                        (s("age"), JSONInteger(1000000)),
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
