#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "lsl.pest"]
struct LanguageSubTagListParser;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_parse_file_date() {
        parses_to!(
            parser: LanguageSubTagListParser,
            input: "File-Date: 2020-07-18\n",
            rule: Rule::file_date,
            tokens: [
                file_date(0, 22, [
                    file_date_field_header(0, 9),
                    date(11, 21)
                ]),
            ]
        )
    }
}
