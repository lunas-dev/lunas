use blve_parser::{DetailedBlock, DetailedMetaData, UseComponentStatement};
use std::collections::HashSet;

use crate::{
    generate_statements::{
        gen_if_blk::gen_render_if_blk_func,
        utils::{create_indent, gen_binary_map_from_bool},
    },
    orig_html_struct::structs::{Node, NodeContent},
    structs::{
        transform_info::{
            sort_if_blocks, ActionAndTarget, CustomComponentBlockInfo, IfBlockInfo, NeededIdName,
            TextNodeRendererGroup, VariableNameAndAssignedNumber,
        },
        transform_targets::{sort_elm_and_reactive_info, NodeAndReactiveInfo},
    },
    transformers::{
        html_utils::{check_html_elms, generate_set_component_statement},
        js_utils::analyze_js,
    },
};

pub fn generate_js_from_blocks(
    blocks: &DetailedBlock,
    no_export: Option<bool>,
    export_name: Option<String>,
    runtime_path: Option<String>,
) -> Result<(String, Option<String>), String> {
    let use_component_statements = blocks
        .detailed_meta_data
        .iter()
        .filter_map(|meta_data| match meta_data {
            DetailedMetaData::UseComponentStatement(use_component) => Some(use_component),
            _ => None,
        })
        .collect::<Vec<&UseComponentStatement>>();

    let component_names = use_component_statements
        .iter()
        .map(|use_component| use_component.component_name.clone())
        .collect::<Vec<String>>();

    let no_export = match no_export.is_none() {
        true => false,
        false => no_export.unwrap(),
    };
    let runtime_path = match runtime_path.is_none() {
        true => "blve/dist/runtime".to_string(),
        false => runtime_path.unwrap(),
    };

    let (variables, variable_names, mut imports, js_output) = analyze_js(blocks);

    for use_component in use_component_statements {
        imports.push(format!(
            "import {} from \"{}\";",
            use_component.component_name, use_component.component_path
        ));
    }

    let imports_string = imports
        .iter()
        .map(|i| format!("\n{}", i))
        .collect::<Vec<String>>()
        .join("");

    // Clone HTML as mutable reference
    let mut needed_id = vec![];

    let mut elm_and_var_relation = vec![];
    let mut action_and_target = vec![];
    let mut if_blocks_info = vec![];
    let mut custom_component_blocks_info = vec![];
    let mut text_node_renderer = vec![];

    let mut new_node = Node::new_from_dom(&blocks.detailed_language_blocks.dom)?;

    // Analyze HTML
    check_html_elms(
        &variable_names,
        &component_names,
        &mut new_node,
        &mut needed_id,
        &mut elm_and_var_relation,
        &mut action_and_target,
        None,
        &mut vec![],
        &mut if_blocks_info,
        &mut custom_component_blocks_info,
        &mut text_node_renderer,
        &vec![],
        &vec![0],
        1,
    )?;

    sort_if_blocks(&mut if_blocks_info);
    sort_elm_and_reactive_info(&mut elm_and_var_relation);

    // TODO: reconsider about this unwrap
    let new_elm = match new_node.content {
        NodeContent::Element(elm) => elm,
        _ => panic!(),
    };

    // Generate JavaScript
    let html_insert = generate_set_component_statement(&new_elm);
    let mut codes = vec![js_output, html_insert];

    // Generate AfterMount
    let mut after_mount_codes = vec![];
    let ref_getter_expression = gen_ref_getter_from_needed_ids(&needed_id);
    after_mount_codes.push(ref_getter_expression);
    let if_block_refs = generate_if_block_ref_var_decl(&if_blocks_info, &needed_id);
    after_mount_codes.extend(if_block_refs);
    let txt_node_renderer = TextNodeRendererGroup::new(
        &if_blocks_info,
        &text_node_renderer,
        &custom_component_blocks_info,
    );
    let create_anchor_statements = gen_create_anchor_statements(&txt_node_renderer, &vec![]);
    after_mount_codes.extend(create_anchor_statements);
    let event_listener_codes = create_event_listener(action_and_target);
    after_mount_codes.extend(event_listener_codes);
    let render_if = gen_render_if_blk_func(&if_blocks_info, &needed_id);
    after_mount_codes.extend(render_if);
    let render_component = gen_render_custom_component_statements(&custom_component_blocks_info);
    after_mount_codes.extend(render_component);
    after_mount_codes.push("this.blkUpdateMap = 0".to_string());
    let update_func_code = gen_on_update_func(elm_and_var_relation, variables, if_blocks_info);
    after_mount_codes.push(update_func_code);
    let after_mount_code = after_mount_codes
        .iter()
        .map(|c| create_indent(c))
        .collect::<Vec<String>>()
        .join("\n");
    let after_mount_func_code = format!(
        r#"__BLVE_AFTER_MOUNT(function () {{
{}
}});
"#,
        after_mount_code
    );
    codes.push(after_mount_func_code);

    codes.push("return __BLVE_COMPONENT_RETURN;".to_string());

    let full_js_code = gen_full_code(no_export, export_name, runtime_path, imports_string, codes);
    let css_code = blocks.detailed_language_blocks.css.clone();

    Ok((full_js_code, css_code))
}

fn gen_full_code(
    no_export: bool,
    export_name: Option<String>,
    runtime_path: String,
    imports_string: String,
    codes: Vec<String>,
) -> String {
    let func_decl = if no_export {
        format!("const {} = ", export_name.unwrap_or("App".to_string()))
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
        r#"import {{ __BLVE_ADD_EV_LISTENER, __BLVE_ESCAPE_HTML, __BLVE_GET_ELM_REFS, __BLVE_INIT_COMPONENT, __BLVE_REPLACE_INNER_HTML, __BLVE_REPLACE_TEXT, __BLVE_REPLACE_ATTR, __BLVE_INSERT_EMPTY, __BLVE_INSERT_CONTENT, __CREATE_BLVE_ELEMENT }} from "{}";{}

{}function() {{
    const {{ __BLVE_SET_COMPONENT_ELEMENT, __BLVE_UPDATE_COMPONENT, __BLVE_COMPONENT_RETURN, __BLVE_AFTER_MOUNT, __BLVE_REACTIVE, __BLVE_RENDER_IF_BLOCK, __BLVE_CREATE_IF_BLOCK }} = __BLVE_INIT_COMPONENT();
{}
}}"#,
        runtime_path, imports_string, func_decl, code,
    )
}

fn gen_ref_getter_from_needed_ids(needed_ids: &Vec<NeededIdName>) -> String {
    let mut ref_getter_str = "const [".to_string();
    ref_getter_str.push_str(
        needed_ids
            .iter()
            .filter(|id: &&NeededIdName| id.ctx.len() == 0)
            .map(|id| format!("__BLVE_{}_REF", id.node_id))
            .collect::<Vec<String>>()
            .join(", ")
            .as_str(),
    );
    ref_getter_str.push_str("] = __BLVE_GET_ELM_REFS([");
    ref_getter_str.push_str(
        needed_ids
            .iter()
            .filter(|id: &&NeededIdName| id.ctx.len() == 0)
            .map(|id| format!("\"{}\"", id.id_name))
            .collect::<Vec<String>>()
            .join(", ")
            .as_str(),
    );
    let delete_id_bool_map = needed_ids
        .iter()
        .filter(|id: &&NeededIdName| id.ctx.len() == 0)
        .map(|id| id.to_delete)
        .collect::<Vec<bool>>();
    let delete_id_map = gen_binary_map_from_bool(delete_id_bool_map);
    ref_getter_str.push_str(format!("], {map});", map = delete_id_map).as_str());
    ref_getter_str
}

fn generate_if_block_ref_var_decl(
    if_blocks_info: &Vec<IfBlockInfo>,
    needed_id: &Vec<NeededIdName>,
) -> Vec<String> {
    let mut codes = vec![];
    if if_blocks_info.len() > 0 {
        let mut variables_to_declare = HashSet::new();
        for if_block_info in if_blocks_info.iter() {
            variables_to_declare.insert(if_block_info.if_block_id.clone());
            let new_ctx_under_if = {
                let mut ctx = if_block_info.ctx.clone();
                ctx.push(if_block_info.if_block_id.clone());
                ctx
            };
            for needed_id in needed_id.iter() {
                if needed_id.ctx == new_ctx_under_if {
                    variables_to_declare.insert(needed_id.node_id.clone());
                }
            }
        }

        if variables_to_declare.len() != 0 {
            let decl = format!(
                "let {};",
                itertools::join(
                    variables_to_declare
                        .iter()
                        .map(|v| format!("__BLVE_{}_REF", v)),
                    ", "
                )
            );
            codes.push(decl);
        }
    }
    return codes;
}

fn create_event_listener(actions_and_targets: Vec<ActionAndTarget>) -> Vec<String> {
    let mut result = vec![];
    for action_and_target in actions_and_targets {
        result.push(format!(
            "__BLVE_ADD_EV_LISTENER(__BLVE_{}_REF, \"{}\", {});",
            action_and_target.target,
            action_and_target.action_name,
            action_and_target.action.to_string()
        ));
    }
    result
}

fn gen_on_update_func(
    elm_and_variable_relations: Vec<NodeAndReactiveInfo>,
    variable_name_and_assigned_numbers: Vec<VariableNameAndAssignedNumber>,
    if_blocks_infos: Vec<IfBlockInfo>,
) -> String {
    let mut replace_statements = vec![];

    for (index, if_block_info) in if_blocks_infos.iter().enumerate() {
        let if_blk_rendering_cond = if if_block_info.ctx.len() != 0 {
            format!(
                "(!((this.blkRenderedMap & {0}) ^ {0})) && ",
                if_block_info.generate_ctx_num(&if_blocks_infos)
            )
        } else {
            "".to_string()
        };

        let dep_vars = &if_block_info.condition_dep_vars;

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
            "{}this.valUpdateMap & {} && ( {} ? {} : ({}, {}, {}) );",
            if_blk_rendering_cond,
            combined_number,
            if_block_info.condition,
            format!("__BLVE_RENDER_{}_ELM()", &if_block_info.if_block_id),
            format!("__BLVE_{}_REF.remove()", &if_block_info.if_block_id),
            format!("__BLVE_{}_REF = null", &if_block_info.if_block_id),
            format!("this.blkRenderedMap ^= {}", index + 1),
        ));
    }

    for elm_and_variable_relation in elm_and_variable_relations {
        match elm_and_variable_relation {
            NodeAndReactiveInfo::ElmAndReactiveAttributeRelation(elm_and_attr_relation) => {
                let _elm_and_attr_relation = elm_and_attr_relation.clone();
                for c in elm_and_attr_relation.reactive_attr {
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

                    let if_blk_rendering_cond = if elm_and_attr_relation.ctx.len() != 0 {
                        format!(
                            "(!((this.blkRenderedMap & {0}) ^ {0})) && ",
                            _elm_and_attr_relation.generate_ctx_num(&if_blocks_infos)
                        )
                    } else {
                        "".to_string()
                    };

                    replace_statements.push(format!(
                        "{}this.valUpdateMap & {:?} && __BLVE_REPLACE_ATTR(\"{}\", {}, __BLVE_{}_REF);",
                        if_blk_rendering_cond,
                        get_combined_binary_number(dep_vars_assined_numbers),
                        c.attribute_key,
                        c.content_of_attr,
                        elm_and_attr_relation.elm_id
                    ));
                }
            }
            NodeAndReactiveInfo::ElmAndVariableRelation(elm_and_variable_relation) => {
                let depending_variables = elm_and_variable_relation.dep_vars.clone();
                let target_id = elm_and_variable_relation.elm_id.clone();

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
                let under_if_blk = elm_and_variable_relation.ctx.len() != 0;

                let if_blk_rendering_cond = if under_if_blk {
                    format!(
                        "(!((this.blkRenderedMap & {0}) ^ {0})) && ",
                        elm_and_variable_relation.generate_ctx_num(&if_blocks_infos)
                    )
                } else {
                    "".to_string()
                };

                let combined_number = get_combined_binary_number(dep_vars_assined_numbers);

                let to_update_cond = if under_if_blk {
                    format!(
                        "(this.valUpdateMap & {:?} && ((this.blkUpdateMap & {1}) ^ {1}) )",
                        combined_number,
                        elm_and_variable_relation.generate_ctx_num(&if_blocks_infos)
                    )
                } else {
                    format!("this.valUpdateMap & {:?}", combined_number)
                };

                replace_statements.push(format!(
                    "{}{} && __BLVE_REPLACE_TEXT(`{}`, __BLVE_{}_REF);",
                    if_blk_rendering_cond,
                    to_update_cond,
                    elm_and_variable_relation.content_of_element.trim(),
                    target_id
                ));
            }
            NodeAndReactiveInfo::TextAndVariableContentRelation(txt_and_var_content) => {
                // TODO: Elementとほとんど同じなので、共通化

                let depending_variables = txt_and_var_content.dep_vars.clone();
                let target_id = txt_and_var_content.text_node_id.clone();

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
                let under_if_blk = txt_and_var_content.ctx.len() != 0;

                let if_blk_rendering_cond = if under_if_blk {
                    format!(
                        "(!((this.blkRenderedMap & {0}) ^ {0})) && ",
                        txt_and_var_content.generate_ctx_num(&if_blocks_infos)
                    )
                } else {
                    "".to_string()
                };

                let combined_number = get_combined_binary_number(dep_vars_assined_numbers);

                let to_update_cond = if under_if_blk {
                    format!(
                        "(this.valUpdateMap & {:?} && ((this.blkUpdateMap & {1}) ^ {1}) )",
                        combined_number,
                        txt_and_var_content.generate_ctx_num(&if_blocks_infos)
                    )
                } else {
                    format!("this.valUpdateMap & {:?}", combined_number)
                };

                replace_statements.push(format!(
                    "{}{} && replaceText(`{}`, {}Text);",
                    if_blk_rendering_cond,
                    to_update_cond,
                    txt_and_var_content.content_of_element.trim(),
                    target_id
                ));
            }
        }
    }

    let code = replace_statements
        .iter()
        .map(|c| create_indent(c))
        .collect::<Vec<String>>()
        .join("\n");

    let result = format!(
        r#"__BLVE_UPDATE_COMPONENT(function () {{
{code}
}});"#,
        code = code
    );

    result
}

fn gen_create_anchor_statements(
    text_node_renderer: &TextNodeRendererGroup,
    ctx_condition: &Vec<String>,
) -> Vec<String> {
    let mut create_anchor_statements = vec![];
    for render in &text_node_renderer.renderers {
        match render {
            crate::structs::transform_info::TextNodeRenderer::ManualRenderer(txt_renderer) => {
                if &txt_renderer.ctx != ctx_condition {
                    continue;
                }
                let anchor_id = match &txt_renderer.target_anchor_id {
                    Some(anchor_id) => format!("__BLVE_{}_REF", anchor_id),
                    None => "null".to_string(),
                };
                let create_anchor_statement = format!(
                    "const {}Text = __BLVE_INSERT_CONTENT(`{}`,__BLVE_{}_REF,{});",
                    &txt_renderer.text_node_id,
                    &txt_renderer.content.trim(),
                    &txt_renderer.parent_id,
                    anchor_id
                );
                create_anchor_statements.push(create_anchor_statement);
            }
            crate::structs::transform_info::TextNodeRenderer::IfBlockRenderer(if_block) => {
                match if_block.distance_to_next_elm > 1 {
                    true => {
                        if &if_block.ctx != ctx_condition {
                            continue;
                        }
                        let anchor_id = match &if_block.target_anchor_id {
                            Some(anchor_id) => format!("__BLVE_{}_REF", anchor_id),
                            None => "null".to_string(),
                        };
                        let create_anchor_statement = format!(
                            "const __BLVE_{}_ANCHOR = __BLVE_INSERT_EMPTY(__BLVE_{}_REF,{});",
                            if_block.if_block_id, if_block.parent_id, anchor_id
                        );
                        create_anchor_statements.push(create_anchor_statement);
                    }
                    false => {}
                }
            }
            crate::structs::transform_info::TextNodeRenderer::CustomComponentRenderer(
                custom_component,
            ) => match custom_component.distance_to_next_elm > 1 {
                true => {
                    if &custom_component.ctx != ctx_condition {
                        continue;
                    }
                    let anchor_id = match &custom_component.target_anchor_id {
                        Some(anchor_id) => format!("__BLVE_{}_REF", anchor_id),
                        None => "null".to_string(),
                    };
                    let create_anchor_statement = format!(
                        "const __BLVE_{}_ANCHOR = __BLVE_INSERT_EMPTY(__BLVE_{}_REF,{});",
                        custom_component.custom_component_block_id,
                        custom_component.parent_id,
                        anchor_id
                    );
                    create_anchor_statements.push(create_anchor_statement);
                }
                false => {}
            },
        }
    }
    create_anchor_statements
}

fn gen_render_custom_component_statements(
    custom_component_block_info: &Vec<CustomComponentBlockInfo>,
) -> Vec<String> {
    let mut render_custom_statements = vec![];

    for custom_component_block in custom_component_block_info.iter() {
        if custom_component_block.have_sibling_elm {
            match custom_component_block.distance_to_next_elm > 1 {
                true => {
                    render_custom_statements.push(format!(
                        "const __BLVE_{}_COMP = {}().insertBefore(__BLVE_{}_REF, __BLVE_{}_ANCHOR);",
                        custom_component_block.custom_component_block_id,
                        custom_component_block.component_name,
                        custom_component_block.parent_id,
                        custom_component_block.custom_component_block_id
                    ));
                }
                false => {
                    let anchor_ref_name = match &custom_component_block.target_anchor_id {
                        Some(anchor_id) => format!("__BLVE_{}_REF", anchor_id),
                        None => "null".to_string(),
                    };
                    render_custom_statements.push(format!(
                        "const __BLVE_{}_COMP = {}().insertBefore(__BLVE_{}_REF, {});",
                        custom_component_block.custom_component_block_id,
                        custom_component_block.component_name,
                        custom_component_block.parent_id,
                        anchor_ref_name
                    ));
                }
            }
        } else {
            render_custom_statements.push(format!(
                "const __BLVE_{}_COMP = {}().mount(__BLVE_{}_REF);",
                custom_component_block.custom_component_block_id,
                custom_component_block.component_name,
                custom_component_block.parent_id
            ));
        }
    }
    render_custom_statements
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
