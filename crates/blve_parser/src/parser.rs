extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1, not_line_ending},
    combinator::{map, recognize},
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct LanguageBlock {
    pub language_name: String,
    pub content: String,
}

#[derive(Debug, PartialEq)]
pub struct MetaData {
    pub kind: String,
    pub arguments: HashMap<String, String>,
    pub content: String,
}

#[derive(Debug, PartialEq)]
pub enum ParsedItem {
    LanguageBlock(LanguageBlock),
    MetaData(MetaData),
}

fn parse_language_name(input: &str) -> IResult<&str, String> {
    map(recognize(alphanumeric1), String::from)(input)
}

fn parse_content(input: &str) -> IResult<&str, String> {
    map(recognize(not_line_ending), String::from)(input)
}

fn parse_language_block(input: &str) -> IResult<&str, ParsedItem> {
    let (input, language_name) = parse_language_name(input)?;
    let (input, _) = tuple((multispace0, tag(":")))(input)?;
    let (input, _) = multispace1(input)?;

    let (input, content) = recognize(nom::multi::many_till(
        nom::character::complete::anychar,
        alt((nom::combinator::peek(tag("@")), nom::combinator::eof)),
    ))(input)?;

    Ok((
        input,
        ParsedItem::LanguageBlock(LanguageBlock {
            language_name,
            content: content.trim().to_string(),
        }),
    ))
}

fn parse_meta_data(input: &str) -> IResult<&str, ParsedItem> {
    let (input, _) = tag("@")(input)?;
    let (input, kind) = parse_language_name(input)?;
    let (input, (arguments, content)) = alt((
        map(
            pair(
                delimited(
                    tag("("),
                    separated_pair(
                        preceded(tag(""), parse_language_name),
                        preceded(tag(":"), tag("")),
                        parse_content,
                    ),
                    tag(")"),
                ),
                preceded(multispace0, parse_content),
            ),
            |((arg_name, arg_content), content)| {
                let mut arguments = HashMap::new();
                arguments.insert(arg_name, arg_content.to_string());
                (arguments, content)
            },
        ),
        map(pair(multispace0, parse_content), |(_, content)| {
            (HashMap::new(), content)
        }),
    ))(input)?;

    Ok((
        input,
        ParsedItem::MetaData(MetaData {
            kind,
            arguments,
            content,
        }),
    ))
}

pub fn parse(input: &str) -> IResult<&str, Vec<ParsedItem>> {
    let (input, items) = nom::multi::separated_list0(
        multispace1,
        alt((parse_language_block, parse_meta_data)),
    )(input)?;

    Ok((input, items))
}
