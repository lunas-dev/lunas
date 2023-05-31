mod structs;
mod transformers;
use blve_parser::parse_blve_file;
use transformers::utils::search_json;

use crate::transformers::utils::{
    add_strings_to_script, check_html_elms, find_variable_declarations,
};

fn main() {
    // "Hello, world!".to_string()

    let a = r#"
html:
    <h1>Hello Blve!</h1>
    <button @click="increment">${count+count} count ${count} ${ count + count }</button>
script:
    let count = 0
    let count2 = 0
    let count3 = 0
    function increment(){
        count++
        count++
        count++
        count++
        count = count + 200
    }
"#;

    let mut b = parse_blve_file(a).unwrap();

    let (variable_names, js_output) = if let Some(js_block) = b.detailed_language_blocks.js {
        let mut positions = vec![];
        let (variables, str_positions) = find_variable_declarations(&js_block.ast);
        for r in str_positions {
            positions.push(r);
        }
        let variable_names = variables.iter().map(|v| v.name.clone()).collect();
        let result = search_json(&js_block.ast, &variable_names, None);
        for r in result {
            positions.push(r);
        }
        let output = add_strings_to_script(positions, &js_block.raw);
        (variable_names, output)
    } else {
        (vec![], "".to_string())
    };

    let (action_and_target, needed_id, _, elm_and_var_rel) = check_html_elms(
        &variable_names,
        &mut b.detailed_language_blocks.dom.children,
    );

    // const [abcref] = getElmRefs(["abc"], 1);
    println!("{:?}", a);
    println!("{:?}", elm_and_var_rel);
    println!("{:?}", needed_id);

    let k = b.detailed_language_blocks.dom.to_string();

    let html_insert = format!("elm.innerHTML = `{}`;", k);
    // let imports = "import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText } from '../a.js'";
    let full_code = gen_full_code(vec![js_output, html_insert]);
    println!("{}", full_code);
}

fn gen_full_code(codes: Vec<String>) -> String {
    // codesにcreate_indentを適用して、\nでjoinする -> code
    let code = codes
        .iter()
        .map(|c| create_indent(c))
        .collect::<Vec<String>>()
        .join("\n");
    format!(
        r#"
import {{ reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText }} from '../a.js'
export default function(elm) {{
    const refs = [0, false, null];
{code}
}}"#,
        code = code
    )
}

fn create_indent(string: &str) -> String {
    let mut output = "".to_string();
    let indent = "    ";
    for line in string.lines() {
        output.push_str(indent);
        output.push_str(line);
        output.push_str("\n");
    }
    output
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
