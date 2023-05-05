use crate::parsers::utils::{empty_lines, parse_language_name};
use crate::structs::blocks::{LanguageBlock, ParsedItem};
extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::recognize,
    sequence::tuple,
    IResult,
};

pub fn parse_language_block(input: &str) -> IResult<&str, ParsedItem> {
    let (input, _) = empty_lines(input)?;
    let (input, language_name) = parse_language_name(input)?;
    let (input, _) = tuple((multispace0, tag(":")))(input)?;
    let (input, _) = multispace1(input)?;

    let (input, content) = recognize(nom::multi::many_till(
        nom::character::complete::anychar,
        alt((nom::combinator::peek(tag("@")), nom::combinator::eof)),
    ))(input)?;

    // print input and content and language_name
    println!("input: {:?}", input);
    println!("content: {:?}", content);
    println!("language_name: {:?}", language_name);

    Ok((
        input,
        ParsedItem::LanguageBlock(LanguageBlock {
            language_name,
            content: content.trim().to_string(),
        }),
    ))
}

// fn is_space(c: char) -> bool {
//   c == ' ' || c == '\t'
// }

// fn indented_line(input: &str) -> IResult<&str, &str> {
//   let (input, _) = take_while1(is_space)(input)?;
//   let (input, content) = take_while(|c| c != '\n')(input)?;
//   let (input, _) = newline(input)?;
//   Ok((input, content))
// }

// fn unindented_line(input: &str) -> IResult<&str, &str> {
//   let (input, _) = tag("\n")(input)?;
//   let (input, content) = take_while(|c| c != '\n' && !is_space(c))(input)?;
//   let (input, _) = newline(input)?;
//   Ok((input, content))
// }

// fn eof(input: &str) -> IResult<&str, &str> {
//   let (input, _) = nom::combinator::eof(input)?;
//   Ok((input, ""))
// }

// fn indented_text(input: &str) -> IResult<&str, String> {
//     let (input, lines) = many_till(indented_line, alt((unindented_line)))(input)?;
//     let content = lines.0.join("\n");
//     Ok((input, content))
// }
