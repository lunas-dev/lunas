use std::env;

use crate::structs::transform_info::AddStringToPosition;

pub fn add_strings_to_script(
    position_and_strs: Vec<AddStringToPosition>,
    script: &String,
) -> String {
    let mut position_and_strs = position_and_strs.clone();
    position_and_strs.sort_by(|a, b| a.position.cmp(&b.position));
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

pub fn gen_nanoid() -> String {
    if is_testgen() {
        "test".to_string()
    } else {
        let alphabet: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'v', 'u', 'w', 'x', 'y', 'z',
        ];
        nanoid::nanoid!(10, &alphabet)
    }
}

fn is_testgen() -> bool {
    match env::var("BLVE_TEST") {
        Ok(_) => true,
        Err(_) => false,
    }
}

// TODO:SWCでパースする
pub fn append_v_to_vars(input: &str, variables: &[String]) -> (String, Vec<String>) {
    let mut depending_vars = Vec::new();
    let operators = [
        "+", "-", "*", "/", "%", "==", "!=", "<", ">", "<=", ">=", "?", ":", "&&", "||", "${", "}",
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
