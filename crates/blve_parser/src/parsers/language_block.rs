use crate::parsers::utils::{empty_lines, parse_language_name};
use crate::structs::blocks::{LanguageBlock, ParsedItem};
extern crate nom;


use nom::bytes::complete::take_while;
use nom::character::complete::{line_ending};
use nom::combinator::{not, opt, peek};
use nom::multi::many_till;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0},
    sequence::tuple,
    IResult,
};

pub fn parse_language_block(input: &str) -> IResult<&str, ParsedItem> {
    let (input, _) = empty_lines(input)?;
    let (input, language_name) = parse_language_name(input)?;
    let (input, _) = tuple((multispace0, tag(":")))(input)?;
    let (input, _) = tag("\n")(input)?;

    let (input, content) = indented_content(input)?;

    // let (input, content) = recognize(nom::multi::many_till(
    //     nom::character::complete::anychar,
    //     alt((
    //         value(("@", ""), peek(tag("@"))),
    //         value(("EOF", ""), eof),
    //         map(
    //             pair(alphanumeric1, preceded(multispace0, tag(":"))),
    //             |(s1, s2)| (s1, s2),
    //         ),
    //     )),
    // ))(input)?;

    // print input and content and language_name

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

/*

Indent Parser

 */

fn is_space_or_tab(c: char) -> bool {
    c == ' ' || c == '\t'
}

fn is_not_line_ending(c: char) -> bool {
    c != '\n' && c != '\r'
}

// fn is_not_line_ending_or_eof(c:char)->{
//     // is_not_line_ending(c) && c !=
// }

fn indent(input: &str) -> IResult<&str, &str> {
    take_while(is_space_or_tab)(input)
}

fn indented_line<'a>(indentation: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| {
        let (input, ret) = opt(tag("\n"))(input)?;
        if ret != None {
            return Ok((input, ""));
        }
        println!("input is: {}", input);
        let (input, _) = tag(indentation)(input)?;
        let (input, line) = take_while(is_not_line_ending)(input)?;
        let (input, _) = line_ending(input)?;
        Ok((input, line))
    }
}

fn take_empty_lines(input: &str) -> IResult<&str, &str> {
    take_while(|c| c == '\n' || c == '\r')(input)
}

fn content_of_first_line(input: &str) -> IResult<&str, &str> {
    let (input, content) = take_while(is_not_line_ending)(input)?;
    Ok((input, content))
}

// fn not_starting_with_indent<'a>(
//     indentation: &'a str,
// ) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
//     move |input: &'a str| {
//         let (_,_) = peek(not(indent))?;
//     }
// }

fn indented_content(input: &str) -> IResult<&str, String> {
    let (input, _) = take_empty_lines(input)?;
    let (input, initial_indentation) = indent(input)?;
    let (input, content_of_first_line) = content_of_first_line(input)?;

    let (input, _) = line_ending(input)?;

    let (input, (mut lines, _)) = many_till(
        indented_line(initial_indentation),
        peek(not(alt((tag(initial_indentation), tag("\n"))))),
    )(input)?;

    println!("lines: {:?}", lines);

    lines.insert(0, content_of_first_line);

    let joined = lines.join("\n");

    Ok((input, joined))
}
