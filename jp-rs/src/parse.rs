
use pa_rs::parser::*;
use crate::json::JSONValue::{self, *};


struct JSONNullParser;

impl Parse for JSONNullParser {
  type Result = JSONValue;

  fn parse<'b>(&self, input: &'b str) -> ParseResult<'b, Self::Result> {
    str_p("null").map(|_| JSONNull).parse(input)
  }
}

struct JSONBoolParser;

impl Parse for JSONBoolParser {
  type Result = JSONValue;

  fn parse<'b>(&self, input: &'b str) -> ParseResult<'b, Self::Result> {
    bool_p().map(|x| JSONBool(x)).parse(input)
  }
}

struct JSONNumberParser;

impl Parse for JSONNumberParser {
  type Result = JSONValue;

  fn parse<'b>(&self, input: &'b str) -> ParseResult<'b, Self::Result> {
    float_p().map(|x| JSONNumber(x)).parse(input)
  }
}

struct JSONStringParser;

impl Parse for JSONStringParser {
  type Result = JSONValue;

  fn parse<'b>(&self, input: &'b str) -> ParseResult<'b, Self::Result> {    
    dq_str_p().map(|x| JSONString(x)).parse(input)
  }
}

struct JSONArrayParser;

impl Parse for JSONArrayParser {
  type Result = JSONValue;

  fn parse<'b>(&self, input: &'b str) -> ParseResult<'b, Self::Result> {
    list_square_p(JSONValueParser)
      .map(|x| JSONArray(x))
      .parse(input)
  }
}

struct JSONObjectParser;

impl Parse for JSONObjectParser {
  type Result = JSONValue;

  fn parse<'b>(&self, input: &'b str) -> ParseResult<'b, Self::Result> {
    let parse_kv = 
    dq_str_p()
      .sbws()
      .keep(char_p(':'))
      .and(JSONValueParser.sbws());

    char_p('{')
      .drop(parse_kv.sep_by(',').map(|x| JSONObject(x)))
      .keep(char_p('}'))
      .parse(input)
  }
}

pub struct JSONValueParser;

impl Parse for JSONValueParser {
  type Result = JSONValue;

  fn parse<'b>(&self, input: &'b str) -> ParseResult<'b, Self::Result> {
    JSONObjectParser
      .or(JSONArrayParser)
      .map(|x| match x {
        Either::Left(lhs) => lhs,
        Either::Right(rhs) => rhs
      })
      .or(JSONStringParser)
      .map(|x| match x {
        Either::Left(lhs) => lhs,
        Either::Right(rhs) => rhs
      })
      .or(JSONNumberParser)
      .map(|x| match x {
        Either::Left(lhs) => lhs,
        Either::Right(rhs) => rhs
      })
      .or(JSONBoolParser)
      .map(|x| match x {
        Either::Left(lhs) => lhs,
        Either::Right(rhs) => rhs
      })
      .or(JSONNullParser) .map(|x| match x {
        Either::Left(lhs) => lhs,
        Either::Right(rhs) => rhs
      })
      .parse(input)
  }
}