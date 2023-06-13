use std::vec;

use crate::structs::{
    ActionAndTarget, AddStringToPosition, ElmAndReactiveAttributeRelation, ElmAndReactiveInfo,
    ElmAndVariableContentRelation, NeededIdName, ReactiveAttr, VariableNameAndAssignedNumber,
};
use blve_html_parser::{Element, Node};
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

// TODO:dep_vars の使い方を再考する
// RCを使用して、子から親のmutableな変数を参照できるようにする可能性も視野に入れる
pub fn check_html_elms(
    varibale_names: &Vec<String>,
    nodes: &mut Vec<Node>,
    needed_ids: &mut Vec<NeededIdName>,
    elm_and_var_relation: &mut Vec<ElmAndReactiveInfo>,
    actions_and_targets: &mut Vec<ActionAndTarget>,
) -> Vec<String> {
    let node_len = *(&nodes.len().clone()) as u32;
    let mut dep_vars: Vec<String> = vec![];
    for node in nodes {
        dep_vars = match node {
            Node::Element(element) => {
                for (key, action_value) in element.attributes.clone() {
                    // if attrs.name starts with "@"
                    if key.starts_with("@") {
                        let action_name = &key[1..];
                        let id: String = set_id_for_needed_elm(element, needed_ids);
                        if let Some(value) = &&action_value {
                            actions_and_targets.push(ActionAndTarget {
                                action_name: action_name.to_string(),
                                action: value.clone(),
                                target: id,
                            })
                        }
                        element.attributes.remove(&key);
                    } else if key.starts_with(":") {
                        let id: String = set_id_for_needed_elm(element, needed_ids);
                        let attr_ = &key[1..];
                        // if elm_and_var_relation includes elm_id
                        if let Some(elm_and_var_relation) = elm_and_var_relation
                            .iter_mut()
                            .filter_map(|elm_and_var_relation| {
                                if let ElmAndReactiveInfo::ElmAndReactiveAttributeRelation(
                                    elm_and_var_relation,
                                ) = elm_and_var_relation
                                {
                                    Some(elm_and_var_relation)
                                } else {
                                    None
                                }
                            })
                            .find(|elm_and_var_relation| elm_and_var_relation.elm_id == id)
                        {
                            // if elm_and_var_relation includes reactive_attr
                            // if !elm_and_var_relation
                            //     .reactive_attr
                            //     .iter_mut()
                            //     .find(|reactive_attr| reactive_attr.attribute_key == attr_)
                            //     .is_some()
                            // {
                            if let Some(action_value) = action_value {
                                let mut variables = vec![];
                                for variable_name in varibale_names {
                                    if action_value.contains(variable_name) {
                                        variables.push(variable_name.to_string());
                                        // replace variable_name with variable_name.v
                                        let mut new_action_value = action_value.clone();
                                        new_action_value = new_action_value.replace(
                                            variable_name,
                                            &format!("{}.v", variable_name),
                                        );
                                        element.attributes.remove(&key);
                                        element.attributes.insert(
                                            attr_.to_string(),
                                            Some(format!("${{{}}}", new_action_value)),
                                        );
                                    }
                                }
                                elm_and_var_relation.reactive_attr.push(ReactiveAttr {
                                    attribute_key: attr_.to_string(),
                                    content_of_attr: element
                                        .attributes
                                        .get(attr_)
                                        .unwrap()
                                        .clone()
                                        .unwrap(),
                                    variable_names: variables,
                                });
                                element.attributes.remove(&key);
                                element.attributes.insert(
                                    attr_.to_string(),
                                    Some(format!(
                                        "${{{}}}",
                                        element.attributes.get(attr_).unwrap().clone().unwrap()
                                    )),
                                );
                            }
                            // }
                        } else {
                            let mut variables = vec![];
                            for variable_name in varibale_names {
                                if let Some(action_value) = action_value.clone() {
                                    if action_value.contains(variable_name) {
                                        variables.push(variable_name.to_string());
                                        let mut new_action_value = action_value.clone();
                                        new_action_value = new_action_value.replace(
                                            variable_name,
                                            &format!("{}.v", variable_name),
                                        );
                                        element.attributes.remove(&key);
                                        element
                                            .attributes
                                            .insert(attr_.to_string(), Some(new_action_value));
                                    }
                                }
                            }
                            if let Some(_) = action_value.clone() {
                                elm_and_var_relation.push(
                                    ElmAndReactiveInfo::ElmAndReactiveAttributeRelation(
                                        ElmAndReactiveAttributeRelation {
                                            elm_id: id,
                                            reactive_attr: vec![ReactiveAttr {
                                                attribute_key: attr_.to_string(),
                                                content_of_attr: element
                                                    .attributes
                                                    .get(attr_)
                                                    .unwrap()
                                                    .clone()
                                                    .unwrap(),
                                                variable_names: variables,
                                            }],
                                        },
                                    ),
                                );
                            }
                            element.attributes.remove(&key);
                            element.attributes.insert(
                                attr_.to_string(),
                                Some(format!(
                                    "${{{}}}",
                                    element.attributes.get(attr_).unwrap().clone().unwrap()
                                )),
                            );
                        }
                    }
                }
                let var_deps = check_html_elms(
                    varibale_names,
                    &mut element.children,
                    needed_ids,
                    elm_and_var_relation,
                    actions_and_targets,
                );

                if var_deps.len() > 0 {
                    let id: String = set_id_for_needed_elm(element, needed_ids);
                    // TODO:以下のIf文のコーナーケースを考える
                    if element.children.len() == 1 && element.children[0].is_text() {
                        elm_and_var_relation.push(ElmAndReactiveInfo::ElmAndVariableRelation(
                            ElmAndVariableContentRelation {
                                elm_id: id,
                                variable_names: var_deps,
                                content_of_element: element.children[0].as_text().clone(),
                            },
                        ));
                    }
                }
                vec![]
            }
            Node::Text(text) => {
                let dep_vars = replace_text_with_reactive_value(text, varibale_names);
                if node_len == 1 {
                    dep_vars
                } else {
                    vec![]
                }
            }
            _ => vec![],
        };
    }
    dep_vars
}

fn set_id_for_needed_elm(element: &mut Element, needed_ids: &mut Vec<NeededIdName>) -> String {
    if let Some(Some(id)) = element.attributes.get("id") {
        let id = if needed_ids.iter().any(|x| x.id_name == id.clone()) {
            id.clone()
        } else {
            needed_ids.push(NeededIdName {
                id_name: id.clone(),
                to_delete: false,
            });
            id.clone()
        };
        id
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
    }
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

// TODO:SWCでパースする
fn append_v_to_vars(input: &str, variables: &[String]) -> (String, Vec<String>) {
    let mut depending_vars = Vec::new();
    let operators = [
        "+", "-", "*", "/", "%", "==", "!=", "<", ">", "<=", ">=", "?", ":",
    ];
    let mut spaced_input = input.to_string();
    for op in &operators {
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

    let output = parts.join(" "); // Ensure that there's space between parts
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

    #[test]
    fn exploration3() {
        let code = "${interval==null?'start':'clear'}";
        let mut code = code.clone().to_string();
        replace_text_with_reactive_value(&mut code, &vec!["interval".to_string()]);
        assert_eq!(
            code,
            "${escapeHtml(interval.v == null ? 'start' : 'clear')}"
        );
    }
}
