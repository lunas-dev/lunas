use std::{env, sync::Mutex};

use rand::{rngs::StdRng, SeedableRng};
use serde_json::Value;

use crate::structs::transform_info::AddStringToPosition;

// TODO: 綺麗な実装にする
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

use rand::seq::SliceRandom;

use super::utils_swc::parse_with_swc;

lazy_static! {
    pub static ref UUID_GENERATOR: Mutex<UuidGenerator> = Mutex::new(UuidGenerator::new());
}

pub struct UuidGenerator {
    seed: u8,
}

impl UuidGenerator {
    fn new() -> UuidGenerator {
        UuidGenerator { seed: 0 }
    }

    pub fn gen(&mut self) -> String {
        if is_testgen() {
            let seed = [self.seed; 32]; // ここに適当なシード値を設定します。
            let mut rng: StdRng = SeedableRng::from_seed(seed);

            let alphabet: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_$";
            let size = 21;

            let id: String = (0..size)
                .map(|_| {
                    let random_char = alphabet.choose(&mut rng).unwrap();
                    *random_char as char
                })
                .collect();
            self.seed = self.seed + 1;
            id
        } else {
            let alphabet: [char; 54] = [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'v', 'u', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F',
                'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'V', 'U',
                'W', 'X', 'Y', 'Z', '_', '$',
            ];
            nanoid::nanoid!(10, &alphabet)
        }
    }
}

fn is_testgen() -> bool {
    match env::var("BLVE_TEST") {
        Ok(_) => true,
        Err(_) => false,
    }
}

// TODO:SWCでパースする
pub fn append_v_to_vars_in_html(input: &str, variables: &Vec<String>) -> (String, Vec<String>) {
    let parsed = parse_with_swc(&input.to_string());

    let parsed_json = serde_json::to_value(&parsed).unwrap();

    let (positions, depending_vars) = search_json(&parsed_json, &variables);

    let modified_string = add_strings_to_script(positions, &input.to_string());

    (modified_string, depending_vars)
}

pub fn search_json(
    json: &Value,
    variables: &Vec<String>,
) -> (Vec<AddStringToPosition>, Vec<String>) {
    let mut positions = vec![];
    let mut depending_vars = vec![];

    if let Value::Object(obj) = json {
        if obj.contains_key("type") && obj["type"] == Value::String("Identifier".into()) {
            if let Some(Value::String(variable_name)) = obj.get("value") {
                if variables.iter().any(|e| e == variable_name) {
                    if let Some(Value::Object(span)) = obj.get("span") {
                        if let Some(Value::Number(end)) = span.get("end") {
                            positions.push(AddStringToPosition {
                                position: (end.as_u64().unwrap() - 1) as u32,
                                string: ".v".to_string(),
                            });
                            depending_vars.push(variable_name.to_string());
                        }
                    }
                }
            }
        } else {
            for (_key, value) in obj {
                let (result_positions, result_vars) = search_json(value, variables);
                positions.extend(result_positions);
                depending_vars.extend(result_vars);
            }
        }
    } else if let Value::Array(arr) = json {
        for value in arr {
            let (result_positions, result_vars) = search_json(value, variables);
            positions.extend(result_positions);
            depending_vars.extend(result_vars);
        }
    }
    (positions, depending_vars)
}
