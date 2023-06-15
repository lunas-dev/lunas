use std::vec;

use crate::structs::{
    ActionAndTarget, AddStringToPosition, ElmAndVariableRelation, NeededIdName,
    VariableNameAndAssignedNumber,
};
use html_parser::Node;
use serde_json::{Map, Value};
pub fn search_json(
    json: &Value,
    variables: &Vec<String>,
    parent: Option<&Map<String, Value>>,
) -> vec::Vec<AddStringToPosition> {
    if let Value::Object(obj) = json {
        if obj.contains_key("type") && obj["type"] == Value::String("Identifier".into()) {
            if parent.is_some()
                && parent.unwrap().get("type")
                    != Some(&Value::String("VariableDeclarator".to_string()))
            {
                if let Some(Value::String(variable_name)) = obj.get("value") {
                    if variables.iter().any(|e| e == variable_name) {
                        if let Some(Value::Object(span)) = obj.get("span") {
                            if let Some(Value::Number(end)) = span.get("end") {
                                return vec![AddStringToPosition {
                                    position: (end.as_u64().unwrap() - 1) as u32,
                                    string: ".v".to_string(),
                                }];
                            }
                        }
                    }
                }
            }

            return vec![];
        } else {
            let mut result = vec![];
            for (_key, value) in obj {
                let search_result = search_json(value, variables, Some(&obj));
                result.extend(search_result);
            }
            return result;
        }
    } else if let Value::Array(arr) = json {
        let mut result = vec![];
        for value in arr {
            let search_result = search_json(value, variables, None);
            result.extend(search_result);
        }
        return result;
    }
    return vec![];
}

pub fn add_strings_to_script(
    position_and_strs: Vec<AddStringToPosition>,
    script: &String,
) -> String {
    let mut result = String::new();
    let mut last_position = 0;
    for position_and_str in position_and_strs {
        result.push_str(&script[last_position..position_and_str.position as usize]);
        result.push_str(&position_and_str.string);
        last_position = position_and_str.position as usize;
    }
    result.push_str(&script[last_position..]);
    return result;
}

pub fn find_variable_declarations(
    json: &Value,
) -> (
    Vec<VariableNameAndAssignedNumber>,
    vec::Vec<AddStringToPosition>,
) {
    if let Some(Value::Array(body)) = json.get("body") {
        let mut variables = vec![];
        let mut str_positions = vec![];
        let mut num_generator = power_of_two_generator();
        for body_item in body {
            if Some(&Value::String("VariableDeclaration".to_string())) == body_item.get("type") {
                if let Some(Value::Array(declarations)) = body_item.get("declarations") {
                    for declaration in declarations {
                        let name = if let Some(Value::Object(id)) = declaration.get("id") {
                            if let Some(Value::String(name)) = id.get("value") {
                                Some(name.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        // get span
                        let start_and_end =
                            if let Some(Value::Object(init)) = declaration.get("init") {
                                if let Some(Value::Object(span)) = init.get("span") {
                                    if let Some(Value::Number(end)) = span.get("end") {
                                        if let Some(Value::Number(start)) = span.get("start") {
                                            Some((start, end))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                        if let Some(name) = name {
                            if let Some((start, end)) = start_and_end {
                                let variable_num = num_generator();
                                variables.push(VariableNameAndAssignedNumber {
                                    name,
                                    assignment: variable_num,
                                });
                                str_positions.push(AddStringToPosition {
                                    position: (start.as_u64().unwrap() - 1) as u32,
                                    string: "reactiveValue(".to_string(),
                                });
                                str_positions.push(AddStringToPosition {
                                    position: (end.as_u64().unwrap() - 1) as u32,
                                    // string: ", 1, refs)".to_string(),
                                    string: format!(", {}, refs)", variable_num),
                                });
                            }
                        }
                    }
                }
            }
        }
        (variables, str_positions)
    } else {
        (vec![], vec![])
    }

    // let mut result = vec![];
    // if let Value::Object(obj) = json {
    //     if obj.contains_key("type") && obj["type"] == Value::String("VariableDeclarator".into())
    //     {
    //         if let Some(Value::Object(id)) = obj.get("id") {
    //             if let Some(Value::String(name)) = id.get("value") {
    //                 result.push(name.to_string());
    //             }
    //         }
    //     } else {
    //         for (_key, value) in obj {
    //             let search_result = find_variable_declarations(value);
    //             result.extend(search_result);
    //         }
    //     }
    // } else if let Value::Array(arr) = json {
    //     for value in arr {
    //         let search_result = find_variable_declarations(value);
    //         result.extend(search_result);
    //     }
    // }
    // return result;
}

fn power_of_two_generator() -> impl FnMut() -> u32 {
    let mut count = 0;
    move || {
        let result = 2u32.pow(count);
        count += 1;
        result
    }
}

pub fn check_html_elms(
    varibale_names: &Vec<String>,
    nodes: &mut Vec<Node>,
) -> (
    Vec<ActionAndTarget>,
    Vec<NeededIdName>,
    Option<Vec<String>>,
    Vec<ElmAndVariableRelation>,
) {
    let mut action_and_targets = vec![];
    // FIXME: needed_idsは毎回生成するのではなく、mutableな参照を、回帰的に渡すようにする
    let mut needed_ids: Vec<NeededIdName> = vec![];
    let mut elm_and_var_relation = vec![];
    let node_len = *(&nodes.len().clone()) as u32;
    let mut dep_vars: Option<Vec<String>> = None;
    for node in nodes {
        dep_vars = match node {
            Node::Element(element) => {
                for (key, action_value) in element.attributes.clone() {
                    // if attrs.name starts with "@"
                    if key.starts_with("@") {
                        // @{action}=
                        // extract action
                        let action_name = &key[1..];
                        // TODO:関数に切り分ける
                        let id: String = if let Some(Some(id)) = element.attributes.get("id") {
                            if needed_ids.iter().any(|x| x.id_name == id.clone()) {
                                id.clone()
                            } else {
                                needed_ids.push(NeededIdName {
                                    id_name: id.clone(),
                                    to_delete: false,
                                });
                                id.clone()
                            }
                        } else {
                            let new_id = gen_nanoid();
                            element
                                .attributes
                                .insert("id".to_string(), Some(new_id.clone()));
                            needed_ids.push(NeededIdName {
                                id_name: new_id.clone(),
                                to_delete: true,
                            });
                            new_id
                        };
                        if let Some(value) = action_value {
                            action_and_targets.push(ActionAndTarget {
                                action_name: action_name.to_string(),
                                action: value.clone(),
                                target: id,
                            })
                        }
                        element.attributes.remove(&key);
                    }
                }
                let (
                    action_and_targets_of_children,
                    needed_ids_of_c,
                    var_deps,
                    elm_and_var_relation_of_c,
                ) = check_html_elms(varibale_names, &mut element.children);
                action_and_targets.extend(action_and_targets_of_children);
                needed_ids.extend(needed_ids_of_c);
                elm_and_var_relation.extend(elm_and_var_relation_of_c);

                if let Some(var_deps) = var_deps {
                    if var_deps.len() > 0 {
                        // TODO:関数に切り分ける
                        let id: String = if let Some(Some(id)) = element.attributes.get("id") {
                            if needed_ids.iter().any(|x| x.id_name == id.clone()) {
                                id.clone()
                            } else {
                                needed_ids.push(NeededIdName {
                                    id_name: id.clone(),
                                    to_delete: false,
                                });
                                id.clone()
                            }
                        } else {
                            let new_id = gen_nanoid();
                            element
                                .attributes
                                .insert("id".to_string(), Some(new_id.clone()));
                            needed_ids.push(NeededIdName {
                                id_name: new_id.clone(),
                                to_delete: true,
                            });
                            new_id
                        };
                        // TODO:以下のIf文のコーナーケースを考える
                        if element.children.len() == 1 && element.children[0].is_text() {
                            elm_and_var_relation.push(ElmAndVariableRelation {
                                elm_id: id,
                                variable_names: var_deps,
                                content_of_element: element.children[0].as_text().clone(),
                            });
                        }
                    }
                }
                None
            }
            Node::Text(text) => {
                let dep_vars = replace_text_with_reactive_value(text, varibale_names);
                if node_len == 1 {
                    Some(dep_vars)
                } else {
                    None
                }
            }
            _ => None,
        };
    }
    (
        action_and_targets,
        needed_ids,
        dep_vars,
        elm_and_var_relation,
    )
}

fn gen_nanoid() -> String {
    let alphabet: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'v', 'u', 'w', 'x', 'y', 'z',
    ];
    nanoid::nanoid!(10, &alphabet)
}

fn escape_html(s: &str) -> String {
    format!("escapeHtml({})", s)
}

fn append_v_to_vars(input: &str, variables: &[String]) -> (String, Vec<String>) {
    let mut depending_vars = Vec::new();
    let operators = ['+', '-', '*', '/', '%'];
    let mut spaced_input = input.to_string();
    for &op in &operators {
        spaced_input = spaced_input.replace(op, &format!(" {} ", op));
    }

    let parts: Vec<String> = spaced_input
        .split_whitespace()
        .map(|part| {
            let trimmed = part.trim();
            if variables.contains(&trimmed.to_string()) {
                depending_vars.push(trimmed.to_string());
                format!("{}.v", trimmed)
            } else {
                trimmed.to_string()
            }
        })
        .collect();

    let output = parts.join("");
    (output, depending_vars)
}
// FIXME:escapeTextは各バインディングに1つだけでいい
// 現在:${escapeHtml(count.v+count.v)} count ${escapeHtml(count)} ${escapeHtml( count + count )}
// 将来的:${escapeHtml(`${count.v+count.v} count ${count} ${ count + count }`)}
fn replace_text_with_reactive_value(code: &mut String, variables: &Vec<String>) -> Vec<String> {
    let start_tag = "${";
    let end_tag = "}";
    let mut new_code = String::new();
    let mut depending_vars = vec![];
    let mut last_end = 0;

    while let Some(start) = code[last_end..].find(start_tag) {
        let start = start + last_end;
        if let Some(end) = code[start..].find(end_tag) {
            let end = end + start;
            let pre_bracket = &code[last_end..start];
            let in_bracket = &code[start + 2..end];
            let _post_bracket = &code[end + 1..];

            new_code.push_str(pre_bracket);
            new_code.push_str(start_tag);
            let (output, dep_vars) = append_v_to_vars(in_bracket, variables);
            new_code.push_str(&escape_html(&output));
            new_code.push_str(end_tag);

            last_end = end + 1;

            depending_vars.extend(dep_vars);
        }
    }

    new_code.push_str(&code[last_end..]);
    *code = new_code;
    depending_vars
}

// test
#[cfg(test)]
mod tests {
    use super::replace_text_with_reactive_value;

    #[test]
    fn exploration() {
        let code = "escapeHtml(count2.v+count.v)";
        let mut code = code.clone().to_string();
        replace_text_with_reactive_value(
            &mut code,
            &vec!["count".to_string(), "count2".to_string()],
        );
        assert_eq!(code, "escapeHtml(count2.v+count.v)");
    }

    #[test]
    fn exploration2() {
        let code = "escapeHtml( count2.v + count.v )";
        let mut code = code.clone().to_string();
        replace_text_with_reactive_value(
            &mut code,
            &vec!["count".to_string(), "count2".to_string()],
        );
        assert_eq!(code, "escapeHtml( count2.v + count.v )");
    }
}
