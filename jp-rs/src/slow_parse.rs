use pa_rs::slow_parser::*;
use crate::json::JSONValue::{self, *};

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