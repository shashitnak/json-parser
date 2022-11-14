mod json;
mod slow_parse;
mod parse;

pub use json::JSONValue::{self, *};
pub use slow_parse::*;
pub use parse::*;

#[cfg(test)]
mod tests {
    use super::*;
    use pa_rs::parser::Parse;

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

        assert_eq!(parse_json().run(raw_json), Ok(parsed_json.clone()));
        assert_eq!(JSONValueParser.run(raw_json), Ok(parsed_json));
        
        println!("{:?}", JSONValueParser.run(raw_json));
    }
}
