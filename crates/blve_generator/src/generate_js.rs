use std::vec;

use blve_parser::DetailedBlock;

use crate::{
    structs::{
        ActionAndTarget, ElmAndVariableRelation, NeededIdName, VariableNameAndAssignedNumber,
    },
    transformers::utils::{
        add_strings_to_script, check_html_elms, find_variable_declarations, search_json,
    },
};

// TODO: bをimmutableにする&リネームする
pub fn generate_js_from_blocks(b: &mut DetailedBlock) -> String {
    let (variables, variable_names, js_output) =
        if let Some(js_block) = &b.detailed_language_blocks.js {
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
            positions.sort_by(|a, b| a.position.cmp(&b.position));
            let output = add_strings_to_script(positions, &js_block.raw);
            (variables, variable_names, output)
        } else {
            (vec![], vec![], "".to_string())
        };

    let (action_and_target, needed_id, _, elm_and_var_rel) = check_html_elms(
        &variable_names,
        &mut b.detailed_language_blocks.dom.children,
    );

    let k = b.detailed_language_blocks.dom.to_string();

    let html_insert = format!("elm.innerHTML = `{}`;", k);
    // let imports = "import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText } from '../a.js'";
    let ref_getter_expression = gen_ref_getter_from_needed_ids(needed_id);
    let event_listener_codes = create_event_listener(action_and_target);
    let mut codes = vec![js_output, html_insert, ref_getter_expression];
    codes.extend(event_listener_codes);
    let update_func_code = gen_update_func_statement(elm_and_var_rel, variables);
    codes.push(update_func_code);
    let full_code = gen_full_code(codes);

    full_code
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
import {{ reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText }} from 'blve/runtime'
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

fn gen_ref_getter_from_needed_ids(needed_ids: Vec<NeededIdName>) -> String {
    let mut ref_getter_str = "const [".to_string();
    ref_getter_str.push_str(
        needed_ids
            .iter()
            .map(|id| format!("{}Ref", id.id_name))
            .collect::<Vec<String>>()
            .join(",")
            .as_str(),
    );
    ref_getter_str.push_str("] = getElmRefs([");
    ref_getter_str.push_str(
        needed_ids
            .iter()
            .map(|id| format!("\"{}\"", id.id_name))
            .collect::<Vec<String>>()
            .join(",")
            .as_str(),
    );
    let delete_id_bool_map = needed_ids
        .iter()
        .map(|id| id.to_delete)
        .collect::<Vec<bool>>();
    let delete_id_map = gen_binary_map_from_bool(delete_id_bool_map);
    ref_getter_str.push_str(format!("], {map});", map = delete_id_map).as_str());
    ref_getter_str
}

fn gen_binary_map_from_bool(bools: Vec<bool>) -> u32 {
    let mut result = 0;
    for (i, &value) in bools.iter().enumerate() {
        if value {
            result |= 1 << (bools.len() - i - 1);
        }
    }
    result
}

fn create_event_listener(actions_and_targets: Vec<ActionAndTarget>) -> Vec<String> {
    let mut result = vec![];
    for action_and_target in actions_and_targets {
        result.push(format!(
            "addEvListener({}Ref, \"{}\", {});",
            action_and_target.target, action_and_target.action_name, action_and_target.action
        ));
    }
    result
}

fn gen_update_func_statement(
    elm_and_variable_relations: Vec<ElmAndVariableRelation>,
    variable_name_and_assigned_numbers: Vec<VariableNameAndAssignedNumber>,
) -> String {
    let mut replace_statements = vec![];
    for elm_and_variable_relation in elm_and_variable_relations {
        let depending_variables = elm_and_variable_relation.variable_names;
        let target_id = elm_and_variable_relation.elm_id;

        // variable_name_and_assigned_numbers.filter((v)=>v.name in elm_and_variable_relation).map(|v| v.name.clone())
        let dep_vars_assined_numbers = variable_name_and_assigned_numbers
            .iter()
            .filter(|v| {
                depending_variables
                    .iter()
                    .map(|d| *d == v.name)
                    .collect::<Vec<bool>>()
                    .contains(&true)
            })
            .map(|v| v.assignment)
            .collect::<Vec<u32>>();

        let combined_number = get_combined_binary_number(dep_vars_assined_numbers);
        replace_statements.push(format!(
            "refs[0] & {:?} && replaceText(`{}`, {}Ref);",
            combined_number, elm_and_variable_relation.content_of_element, target_id
        ));
    }

    let code = replace_statements
        .iter()
        .map(|c| create_indent(c))
        .collect::<Vec<String>>()
        .join("\n");

    let result = format!(
        r#"refs[2] = genUpdateFunc(() => {{
{code}
}});"#,
        code = code
    );

    result
}

fn get_combined_binary_number(numbers: Vec<u32>) -> u32 {
    let mut result = 0;
    for (_, &value) in numbers.iter().enumerate() {
        result |= value;
    }
    result
}
