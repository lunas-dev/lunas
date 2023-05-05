use crate::structs::blocks::ParsedItem;
extern crate nom;

use crate::parsers::{language_block::parse_language_block, metadata::parse_meta_data};

use nom::{branch::alt, multi::many0, IResult};

pub fn parse(input: &str) -> IResult<&str, Vec<ParsedItem>> {
    // let (input, items) = nom::multi::separated_list0(
    //     multispace1,
    //     alt((parse_language_block, parse_meta_data)),
    // )(input)?;
    let (input, items) = many0(alt((parse_language_block, parse_meta_data)))(input)?;

    Ok((input, items))
}
