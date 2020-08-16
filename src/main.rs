#[macro_use]
extern crate nom;
use nom::{
  IResult,
  bytes::complete::{take_while_m_n},
  combinator::map_res,
};

use chrono::{ NaiveDate, ParseResult};
use std::io;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug,PartialEq)]
enum Type {
    Extlang,
    Grandfathered,
    Language,
    Redundant,
    Region,
    Script,
    Variant
}

impl Type {
    fn from_str(input: &str) -> Option<Self>{
        match input {
            "extlang" => Some(Type::Extlang),
            "grandfathered" => Some(Type::Grandfathered),
            "language" => Some(Type::Language),
            "redundant" => Some(Type::Redundant),
            "region" => Some(Type::Region),
            "script" => Some(Type::Script),
            "variant" => Some(Type::Variant),
            _ => None
        }
    }
}

#[derive(Debug,PartialEq)]
enum Scope {
    Collection,
    Macrolanguage,
    PrivateUse,
    Special
}

#[derive(Debug,PartialEq)]
struct Record {
    added: Option<NaiveDate>,
    deprecated: Option<NaiveDate>,
    scope: Option<Scope>,
    record_type: Option<Type>,
    comments: Option<String>,
    description: Option<String>,
    macrolanguage: Option<String>,
    preferred_value: Option<String>,
    prefix: Option<String>,
    sub_tag: Option<String>,
    suppress_script: Option<String>,
    tag: Option<String>
}

#[derive(Debug,PartialEq)]
struct SubTagList {
    file_date: NaiveDate,
    records: Vec<Record>,
}

fn date_from_str(ymd: &str) -> ParseResult<NaiveDate> {
     NaiveDate::parse_from_str(ymd, "%Y-%m-%d")
}

fn is_date_particle(c: char) -> bool {
    c.is_digit(10) || c == '-'
}

fn date(input: &str) -> IResult<&str, NaiveDate> {
    map_res(
        take_while_m_n(10,10, is_date_particle),
        date_from_str
    )(input)
}

named!(get_file_date_value<&str, NaiveDate>,
    terminated!(preceded!(tag!("File-Date: "), date), char!('\n'))
);


fn type_value(input: &str) -> IResult<&str, Type> {
    map!(
        take_until!("\n"),
        |s| Type::from_str(s)
    )
}

named!(get_type_value<&str, Type>,
    terminated!(preceded!(tag!("Type: "), type_value), char!('\n'))
);

#[allow(dead_code)]
fn parse(to_parse: &str) -> io::Result<SubTagList> {
    Ok(SubTagList {
        file_date: NaiveDate::from_ymd(0, 1, 1),
        records: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file_date() {
        let expected = NaiveDate::from_ymd(2020, 7, 17);
        let to_parse = "File-Date: 2020-07-17\n";
        let actual = get_file_date_value(to_parse);
        match actual {
            Ok((_, some_date)) => assert_eq!(some_date, expected),
            nok => assert!(false, "err: [{:?}]", nok)
        };
    }

    #[test]
    fn test_get_type_value() {
        let expected = Type::Language;
        let to_parse = "Type: language\n";
        let actual = get_type_value(to_parse);
        match actual {
            Ok((_, some_type)) => assert_eq!(some_type, expected),
            nok => assert!(false, "err: [{:?}]", nok)
        };
    }
}
