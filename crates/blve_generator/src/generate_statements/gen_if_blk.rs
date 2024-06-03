use crate::{
    orig_html_struct::structs::NodeContent,
    structs::transform_info::{IfBlockInfo, NeededIdName},
};

use super::utils::{create_indent, gen_binary_map_from_bool};

// TODO: Many of the following functions are similar to top-level component creation functions, such as creating refs and rendering if statements. Consider refactoring them into a single function.
pub fn gen_render_if_blk_func(
    if_block_info: &Vec<IfBlockInfo>,
    needed_ids: &Vec<NeededIdName>,
) -> Vec<String> {
    let mut render_if = vec![];

    for (index, if_block) in if_block_info.iter().enumerate() {
        let (name, js_gen_elm_code_arr) = match &if_block.elm.content {
            NodeContent::Element(elm) => elm.generate_element_on_js(&if_block.if_block_id),
            _ => panic!(),
        };
        let insert_elm = match if_block.distance_to_next_elm > 1 {
            true => format!(
                "__BLVE_{}_REF.insertBefore({}, __BLVE_{}_ANCHOR);",
                if_block.parent_id, name, if_block.if_block_id
            ),
            false => match if_block.target_anchor_id {
                Some(_) => format!(
                    "__BLVE_{}_REF.insertBefore({}, __BLVE_{}_REF);",
                    if_block.parent_id,
                    name,
                    if_block.target_anchor_id.as_ref().unwrap().clone()
                ),
                None => format!(
                    "__BLVE_{}_REF.insertBefore({}, null);",
                    if_block.parent_id, name
                ),
            },
        };

        // TODO:一連の生成コードを、need_idのmethodとして関数にまとめる
        let current_blk_ctx = {
            let mut new_ctx = if_block.ctx.clone();
            new_ctx.push(if_block.if_block_id.clone());
            new_ctx
        };
        let filtered = needed_ids
            .iter()
            .filter(|id: &&NeededIdName| id.ctx == current_blk_ctx)
            .filter(|id: &&NeededIdName| id.node_id != if_block.if_block_id);

        let ref_getter_str = if filtered.clone().count() > 0 {
            // TODO:format!などを使ってもっとみやすいコードを書く
            let mut ref_getter_str = "\n[".to_string();

            ref_getter_str.push_str(
                filtered
                    .clone()
                    .map(|id| format!("__BLVE_{}_REF", id.node_id))
                    .collect::<Vec<String>>()
                    .join(", ")
                    .as_str(),
            );
            ref_getter_str.push_str("] = __BLVE_GET_ELM_REFS([");
            ref_getter_str.push_str(
                filtered
                    .clone()
                    .map(|id| format!("\"{}\"", id.id_name))
                    .collect::<Vec<String>>()
                    .join(",")
                    .as_str(),
            );
            let delete_id_bool_map = needed_ids
                .iter()
                .filter(|id: &&NeededIdName| id.ctx == current_blk_ctx)
                .map(|id| id.to_delete)
                .collect::<Vec<bool>>();
            let delete_id_map = gen_binary_map_from_bool(delete_id_bool_map);
            ref_getter_str.push_str(format!("], {map});", map = delete_id_map).as_str());

            ref_getter_str
        } else {
            "".to_string()
        };

        let children = if_block.find_children(&if_block_info);

        let child_block_rendering_exec = if children.len() != 0 {
            let mut rendering_statement = "\n".to_string();
            let mut child_block_rendering_exec = vec![];
            for child_if in children {
                child_block_rendering_exec.push(format!(
                    "{} && __BLVE_RENDER_{}_ELM()",
                    child_if.condition, &child_if.if_block_id
                ));
            }
            rendering_statement.push_str(child_block_rendering_exec.join("\n").as_str());
            rendering_statement
        } else {
            "".to_string()
        };

        // TODO: 一連の処理を関数にまとめる
        // TODO: CreateIndentを複数行に対応させる
        let js_gen_elm_code = js_gen_elm_code_arr
            .iter()
            .map(|c| create_indent(c))
            .collect::<Vec<String>>()
            .join("\n");
        let blk_num: u64 = (2 as u64).pow(index as u32);
        // TODO: {}の前後に改行があったりなかったりするので、統一する
        render_if.push(format!(
            r#"const __BLVE_RENDER_{}_ELM = () => {{
{}
{}
{}{}{}
}}"#,
            &if_block.if_block_id,
            js_gen_elm_code,
            create_indent(insert_elm.as_str()),
            create_indent(
                format!(
                    "this.blkRenderedMap |= {}, this.blkUpdateMap |= {};",
                    blk_num, blk_num
                )
                .as_str()
            ),
            create_indent(ref_getter_str.as_str()),
            create_indent(child_block_rendering_exec.as_str())
        ));
        if if_block.ctx.len() == 0 {
            render_if.push(format!(
                "{} && __BLVE_RENDER_{}_ELM()",
                if_block.condition, &if_block.if_block_id
            ));
        }
    }
    render_if
}
