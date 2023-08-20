use blve_parser::DetailedBlock;

use crate::{
    orig_html_struct::structs::{Node, NodeContent},
    structs::{
        transform_info::{
            ActionAndTarget, IfBlockInfo, NeededIdName, VariableNameAndAssignedNumber,
        },
        transform_targets::ElmAndReactiveInfo,
    },
    transformers::{html_utils::check_html_elms, js_utils::analyze_js},
};

pub fn generate_js_from_blocks(
    blocks: &DetailedBlock,
    no_export: Option<bool>,
    runtime_path: Option<String>,
) -> Result<(String, Option<String>), String> {
    let no_export = match no_export.is_none() {
        true => false,
        false => no_export.unwrap(),
    };
    let runtime_path = match runtime_path.is_none() {
        true => "blve/dist/runtime".to_string(),
        false => runtime_path.unwrap(),
    };

    // Analyze JavaScript
    let (variables, variable_names, js_output) = analyze_js(blocks);

    // Clone HTML as mutable reference
    let mut needed_id = vec![];

    let mut elm_and_var_relation = vec![];
    let mut action_and_target = vec![];
    let mut if_blocks_info = vec![];

    let mut new_node = Node::new_from_dom(&blocks.detailed_language_blocks.dom)?;

    // Analyze HTML
    check_html_elms(
        &variable_names,
        &mut new_node,
        &mut needed_id,
        &mut elm_and_var_relation,
        &mut action_and_target,
        None,
        &mut vec![],
        &mut if_blocks_info,
        &vec![],
    )?;

    let html_str = new_node.to_string();

    // Generate JavaScript
    let html_insert = format!("elm.innerHTML = `{}`;", html_str);

    let create_anchor_statements = gen_create_anchor_statements(&if_blocks_info);
    let ref_getter_expression = gen_ref_getter_from_needed_ids(needed_id);
    let event_listener_codes = create_event_listener(action_and_target);
    let mut codes = vec![js_output, html_insert, ref_getter_expression];

    if if_blocks_info.len() > 0 {
        let mut decl = "let ".to_string();
        for (index, if_block_info) in if_blocks_info.iter().enumerate() {
            decl.push_str(format!("{}Ref", if_block_info.if_block_id).as_str());
            if index != if_blocks_info.len() - 1 {
                decl.push_str(", ");
            }
        }
        codes.push(decl);
    }

    codes.extend(create_anchor_statements);
    codes.extend(event_listener_codes);
    let render_if = gen_render_if_statements(&if_blocks_info);
    codes.extend(render_if);
    let update_func_code =
        gen_update_func_statement(elm_and_var_relation, variables, if_blocks_info);
    codes.push(update_func_code);
    let full_code = gen_full_code(codes, no_export, runtime_path);
    let css_code = blocks.detailed_language_blocks.css.clone();

    Ok((full_code, css_code))
}

fn gen_full_code(codes: Vec<String>, no_export: bool, runtime_path: String) -> String {
    let func_decl = if no_export {
        "const App = ".to_string()
    } else {
        "export default ".to_string()
    };

    // codesにcreate_indentを適用して、\nでjoinする -> code
    let code = codes
        .iter()
        .map(|c| create_indent(c))
        .collect::<Vec<String>>()
        .join("\n");
    format!(
        r#"import {{ reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr,insertEmpty }} from '{}'

{}function(elm) {{
    const refs = [null, false, 0, 0];
{}
}}"#,
        runtime_path, func_decl, code,
    )
}

// TODO: インデントの種類を入力によって変えられるようにする
fn create_indent(string: &str) -> String {
    let mut output = "".to_string();
    let indent = "    ";
    for line in string.lines() {
        match line == "" {
            true => output.push_str("\n"),
            false => {
                output.push_str(indent);
                output.push_str(line);
                output.push_str("\n");
            }
        }
    }
    output
}

fn gen_ref_getter_from_needed_ids(needed_ids: Vec<NeededIdName>) -> String {
    let mut ref_getter_str = "const [".to_string();
    ref_getter_str.push_str(
        needed_ids
            .iter()
            .filter(|id: &&NeededIdName| id.get_ref)
            .map(|id| format!("{}Ref", id.node_id))
            .collect::<Vec<String>>()
            .join(",")
            .as_str(),
    );
    ref_getter_str.push_str("] = getElmRefs([");
    ref_getter_str.push_str(
        needed_ids
            .iter()
            .filter(|id: &&NeededIdName| id.get_ref)
            .map(|id| format!("\"{}\"", id.id_name))
            .collect::<Vec<String>>()
            .join(",")
            .as_str(),
    );
    let delete_id_bool_map = needed_ids
        .iter()
        .filter(|id: &&NeededIdName| id.get_ref)
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
            action_and_target.target,
            action_and_target.action_name,
            action_and_target.action.to_string()
        ));
    }
    result
}

fn gen_update_func_statement(
    elm_and_variable_relations: Vec<ElmAndReactiveInfo>,
    variable_name_and_assigned_numbers: Vec<VariableNameAndAssignedNumber>,
    if_blocks_info: Vec<IfBlockInfo>,
) -> String {
    let mut replace_statements = vec![];
    for elm_and_variable_relation in elm_and_variable_relations {
        match elm_and_variable_relation {
            ElmAndReactiveInfo::ElmAndReactiveAttributeRelation(a) => {
                for c in a.reactive_attr {
                    let dep_vars_assined_numbers = variable_name_and_assigned_numbers
                        .iter()
                        .filter(|v| {
                            c.variable_names
                                .iter()
                                .map(|d| *d == v.name)
                                .collect::<Vec<bool>>()
                                .contains(&true)
                        })
                        .map(|v| v.assignment)
                        .collect::<Vec<u32>>();

                    replace_statements.push(format!(
                        "refs[2]  & {:?} && replaceAttr(\"{}\", {}, {}Ref);",
                        get_combined_binary_number(dep_vars_assined_numbers),
                        c.attribute_key,
                        c.content_of_attr,
                        a.elm_id
                    ));
                }
            }
            ElmAndReactiveInfo::ElmAndVariableRelation(elm_and_variable_relation) => {
                let depending_variables = elm_and_variable_relation.variable_names;
                let target_id = elm_and_variable_relation.elm_id;

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
                    "refs[2] & {:?} && replaceText(`{}`, {}Ref);",
                    combined_number, elm_and_variable_relation.content_of_element, target_id
                ));
            }
        }
    }

    for if_block_info in if_blocks_info {
        let dep_vars = if_block_info.condition_dep_vars;

        // TODO: データバインディングと同じコードを使っているので共通化する
        let dep_vars_assined_numbers = variable_name_and_assigned_numbers
            .iter()
            .filter(|v| {
                dep_vars
                    .iter()
                    .map(|d| *d == v.name)
                    .collect::<Vec<bool>>()
                    .contains(&true)
            })
            .map(|v| v.assignment)
            .collect::<Vec<u32>>();

        let combined_number = get_combined_binary_number(dep_vars_assined_numbers);

        replace_statements.push(format!(
            "refs[2] & {} && ( {} ? {} : ({}, {}) )",
            combined_number,
            if_block_info.condition,
            format!("render{}Elm()", &if_block_info.if_block_id),
            format!("{}Ref.remove()", &if_block_info.if_block_id),
            format!("{}Ref = null", &if_block_info.if_block_id)
        ));
    }

    let code = replace_statements
        .iter()
        .map(|c| create_indent(c))
        .collect::<Vec<String>>()
        .join("\n");

    let result = format!(
        r#"refs[0] = genUpdateFunc(() => {{
{code}
}});"#,
        code = code
    );

    result
}

fn gen_create_anchor_statements(if_block_info: &Vec<IfBlockInfo>) -> Vec<String> {
    let mut create_anchor_statements = vec![];
    for if_block in if_block_info {
        match if_block.distance > 1 {
            true => {
                if if_block.ctx.len() > 0 {
                    continue;
                }
                let anchor_id = match &if_block.target_anchor_id {
                    Some(anchor_id) => format!("{}Ref", anchor_id),
                    None => "null".to_string(),
                };
                let create_anchor_statement = format!(
                    "const {}Anchor = insertEmpty({}Ref,{});",
                    if_block.if_block_id, if_block.parent_id, anchor_id
                );
                create_anchor_statements.push(create_anchor_statement);
            }
            false => {}
        }
    }
    create_anchor_statements
}

fn gen_render_if_statements(if_block_info: &Vec<IfBlockInfo>) -> Vec<String> {
    let mut render_if = vec![];

    for if_block in if_block_info {
        let (name, js_gen_elm_code_arr) = match &if_block.elm.content {
            NodeContent::Element(elm) => elm.generate_element_on_js(&if_block.if_block_id),
            _ => panic!(),
        };
        let insert_elm = match if_block.distance > 1 {
            true => format!(
                "{}Ref.insertBefore({}, {}Anchor);",
                if_block.parent_id, name, if_block.if_block_id
            ),
            false => match if_block.target_anchor_id {
                Some(_) => format!(
                    "{}Ref.insertBefore({}, {}Ref);",
                    if_block.parent_id,
                    name,
                    if_block.target_anchor_id.as_ref().unwrap().clone()
                ),
                None => format!("{}Ref.insertBefore({}, null);", if_block.parent_id, name),
            },
        };
        // TODO: 一連の処理を関数にまとめる
        let js_gen_elm_code = js_gen_elm_code_arr
            .iter()
            .map(|c| create_indent(c))
            .collect::<Vec<String>>()
            .join("\n");
        render_if.push(format!(
            r#"const render{}Elm = () => {{
{}
{}
}}"#,
            &if_block.if_block_id,
            js_gen_elm_code,
            create_indent(insert_elm.as_str())
        ));
        render_if.push(format!(
            "{} && render{}Elm()",
            if_block.condition, &if_block.if_block_id
        ));
    }
    render_if
}

/// Returns a binary number that is the result of ORing all the numbers in the argument.
/// ```
/// let numbers = vec![0b0001, 0b0010, 0b0100];
/// let result = get_combined_binary_number(numbers);
/// assert_eq!(result, 0b0111);
/// ```
fn get_combined_binary_number(numbers: Vec<u32>) -> u32 {
    let mut result = 0;
    for (_, &value) in numbers.iter().enumerate() {
        result |= value;
    }
    result
}
