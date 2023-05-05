use crate::parsers::utils::{empty_lines, parse_content, parse_language_name};
use crate::structs::blocks::{MetaData, ParsedItem};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::map,
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};
use std::collections::HashMap;

pub fn parse_meta_data(input: &str) -> IResult<&str, ParsedItem> {
    let (input, _) = empty_lines(input)?;

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
