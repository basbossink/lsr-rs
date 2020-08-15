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

enum Type {
    Extlang,
    Grandfathered,
    Language,
    Redundant,
    Region,
    Script,
    Variant
}

enum Scope {
    Collection,
    Macrolanguage,
    PrivateUse,
    Special
}

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
}
