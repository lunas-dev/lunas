use std::vec;

use blve_parser::DetailedBlock;
use serde_json::{Map, Value};

use crate::structs::transform_info::{AddStringToPosition, VariableNameAndAssignedNumber};

use super::utils::add_strings_to_script;

pub fn analyze_js(
    blocks: &DetailedBlock,
) -> (Vec<VariableNameAndAssignedNumber>, Vec<String>, String) {
    if let Some(js_block) = &blocks.detailed_language_blocks.js {
        let mut positions = vec![];
        // find all variable declarations
        let (variables, str_positions) = find_variable_declarations(&js_block.ast);
        // add all variable declarations to positions to add custom variable declaration function
        positions.extend(str_positions);
        let variable_names = variables.iter().map(|v| v.name.clone()).collect();
        let result = search_json(&js_block.ast, &variable_names, None);
        positions.extend(result);
        let output = add_strings_to_script(positions, &js_block.raw);
        (variables, variable_names, output)
    } else {
        (vec![], vec![], "".to_string())
    }
}

// Finds all variable declarations in a javascript file and returns a vector of VariableNameAndAssignedNumber structs
fn find_variable_declarations(
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
}

fn power_of_two_generator() -> impl FnMut() -> u32 {
    let mut count = 0;
    move || {
        let result = 2u32.pow(count);
        count += 1;
        result
    }
}

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
