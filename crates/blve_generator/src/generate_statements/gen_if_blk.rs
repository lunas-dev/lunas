use crate::{
    generate_js::gen_ref_getter_from_needed_ids,
    orig_html_struct::structs::NodeContent,
    structs::transform_info::{IfBlockInfo, NeededIdName},
    transformers::html_utils::create_blve_internal_component_statement,
};

use super::utils::create_indent;

// TODO: Many of the following functions are similar to top-level component creation functions, such as creating refs and rendering if statements. Consider refactoring them into a single function.
pub fn gen_render_if_blk_func(
    if_block_info: &Vec<IfBlockInfo>,
    needed_ids: &Vec<NeededIdName>,
) -> Vec<String> {
    let mut render_if = vec![];

    for if_block in if_block_info.iter() {
        // create element
        let create_internal_element_statement = match &if_block.node.content {
            NodeContent::Element(elm) => {
                create_blve_internal_component_statement(elm, "__CREATE_BLVE_ELEMENT")
            }
            _ => panic!(),
        };

        // TODO:一連の生成コードを、need_idのmethodとして関数にまとめる
        let ref_getter_str = gen_ref_getter_from_needed_ids(
            needed_ids,
            &Some(if_block),
            &Some(&if_block.ctx_under_if),
        );

        // if there are children if block under the if block, render them
        let children = if_block.find_children(&if_block_info);

        let mut rendering_statement = "".to_string();
        rendering_statement.push_str(ref_getter_str.as_str());
        let child_block_rendering_exec = if children.len() != 0 {
            let mut child_block_rendering_exec = vec![];
            for child_if in children {
                child_block_rendering_exec.push(format!(
                    "\n{} && __BLVE_RENDER_IF_BLOCK(\"{}\");",
                    child_if.condition, &child_if.if_blk_id
                ));
            }
            child_block_rendering_exec
        } else {
            vec![]
        };
        rendering_statement.push_str(child_block_rendering_exec.join("\n").as_str());

        let name_of_parent_of_if_blk = format!("__BLVE_{}_REF", if_block.parent_id);
        let name_of_anchor_of_if_blk = match if_block.distance_to_next_elm > 1 {
            true => format!("__BLVE_{}_ANCHOR", if_block.if_blk_id),
            false => match if_block.target_anchor_id {
                Some(_) => format!(
                    "__BLVE_{}_REF",
                    if_block.target_anchor_id.as_ref().unwrap().clone()
                ),
                None => format!("null"),
            },
        };

        let if_on_create = match rendering_statement == "" {
            true => "() => {}".to_string(),
            false => format!(
                r#"function() {{
{}
}}"#,
                create_indent(rendering_statement.as_str()),
            ),
        };

        let create_if_func_inside = format!(
            r#""{}",
()=>{},
()=>[{},{}],
{},"#,
            if_block.target_if_blk_id,
            create_internal_element_statement,
            name_of_parent_of_if_blk,
            name_of_anchor_of_if_blk,
            if_on_create,
        );

        let create_if_func = format!(
            r#"__BLVE_CREATE_IF_BLOCK(
{}
) "#,
            create_indent(create_if_func_inside.as_str())
        );

        render_if.push(create_if_func);
        if if_block.ctx_over_if.len() == 0 {
            render_if.push(format!(
                "{} && __BLVE_RENDER_IF_BLOCK(\"{}\")",
                if_block.condition, &if_block.if_blk_id
            ));
        }
    }
    render_if
}
