use std::collections::HashMap;

use crate::structs::blocks::{LanguageBlock, MetaData, ParsedItem};
extern crate nom;

use crate::parsers::{language_block::parse_language_block, metadata::parse_meta_data};
use crate::structs::detailed_blocks::DetailedBlock;
use crate::structs::detailed_language_blocks::DetailedLanguageBlocks;
use crate::structs::detailed_meta_data::DetailedMetaData;

use html_parser::Dom;
use nom::{branch::alt, multi::many0, IResult};
use rome_js_parser;
use rome_js_syntax::SourceType;

pub fn parse2<'a>(input: Vec<ParsedItem>) -> Result<DetailedBlock, &'a str> {
    let variant_a_values: Vec<LanguageBlock> = input
        .clone()
        .into_iter()
        .filter_map(|e| match e {
            ParsedItem::LanguageBlock(bl) => Some(bl),
            _ => None,
        })
        .collect();
    let lang_blocks = parse_language_blocks(variant_a_values)?;

    let detailed_meta_data = input
        .into_iter()
        .filter_map(|e| match e {
            ParsedItem::MetaData(meta) => Some(meta),
            _ => None,
        })
        .map(|e| DetailedMetaData::from_simple_meta_data(e))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(DetailedBlock {
        detailed_meta_data: detailed_meta_data,
        detailed_language_blocks: lang_blocks,
    })
}

fn parse_language_blocks<'a>(blks: Vec<LanguageBlock>) -> Result<DetailedLanguageBlocks, &'a str> {
    let mut hm = HashMap::new();
    for block in &blks {
        let language_name: &str = &block.language_name.as_str();
        // if language_name is not one of 'html', 'style', 'script'
        if language_name != "html" && language_name != "style" && language_name != "script" {
            return Err("Invalid language name");
        }
        if hm.contains_key(language_name) {
            return Err("Duplicate language name");
        }
        let content = block.content.clone();

        hm.insert(language_name.clone(), content);
    }

    let html = hm.get("html");
    if html == None {
        return Err("Missing html block");
    }
    let parsed_html_dom_result = Dom::parse(html.unwrap());
    match parsed_html_dom_result {
        Ok(parsed_html) => {
            let css = hm.get("style");
            let js = hm.get("script");
            let parsed_js = match js {
                Some(js) => {
                    let module = SourceType::ts();

                    let parsed_js = rome_js_parser::parse(js, module);
                    if parsed_js.has_errors() {
                        // let err = parsed_js.errors.
                        return Err("Invalid ts block");
                    }
                    Some(parsed_js.tree())
                }
                None => None,
            };
            let str_css = match css {
                Some(css) => Some(css.to_string()),
                None => None,
            };
            Ok(DetailedLanguageBlocks {
                dom: parsed_html,
                css: str_css,
                js: parsed_js,
            })
        }
        Err(_) => return Err("Invalid html block"),
    }
    // Ok(DetailedLanguageBlocks {
    //     dom: parsed_html,
    //     css: (),
    //     js: (),
    // })
}
