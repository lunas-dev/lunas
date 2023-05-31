mod structs;
mod transformers;
use blve_parser::parse_blve_file;
use transformers::utils::search_json;

use crate::transformers::utils::add_strings_to_script;

pub fn get_parse_result() {
    // "Hello, world!".to_string()

    let a = r#"
html:
    <h1>Hello Blve!</h1>
    <button @click="increment">${count}</button>
script:
    let count = 0
    function increment(){
        count++
    }
"#;

    let b = parse_blve_file(a).unwrap();

    if let Some(js_block) = b.detailed_language_blocks.js {
        let result = search_json(&js_block.ast, &vec!["count".to_string()], None);
        let mut positions = vec![];
        for r in result {
            positions.push(r);
            // if let structs::TransformAnalysisResult::AddDotV(dotv) = r {
            // }
        }
        let output = add_strings_to_script(positions, &js_block.raw);
        println!("{}", output);
    }
}

// mod transformers;

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
