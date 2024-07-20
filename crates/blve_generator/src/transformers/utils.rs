use crate::structs::transform_info::TransformInfo;
use rand::{rngs::StdRng, SeedableRng};
use std::{env, sync::Mutex};

// TODO: 綺麗な実装にする
pub fn add_or_remove_strings_to_script(
    position_and_strs: Vec<TransformInfo>,
    script: &String,
) -> String {
    let mut transformers = position_and_strs.clone();
    transformers.sort_by(|a, b| {
        let a = match a {
            TransformInfo::AddStringToPosition(a) => a.position,
            TransformInfo::RemoveStatement(a) => a.start_position,
            TransformInfo::ReplaceText(a) => a.start_position,
        };
        let b = match b {
            TransformInfo::AddStringToPosition(b) => b.position,
            TransformInfo::RemoveStatement(b) => b.start_position,
            TransformInfo::ReplaceText(b) => b.start_position,
        };
        a.cmp(&b)
    });
    let mut result = String::new();
    let mut last_position = 0;
    for transform in transformers {
        match transform {
            TransformInfo::AddStringToPosition(add) => {
                result.push_str(&script[last_position..add.position as usize]);
                result.push_str(&add.string);
                last_position = add.position as usize;
            }
            TransformInfo::RemoveStatement(remove) => {
                result.push_str(&script[last_position..remove.start_position as usize]);
                last_position = remove.end_position as usize;
            }
            TransformInfo::ReplaceText(replace) => {
                result.push_str(&script[last_position..replace.start_position as usize]);
                result.push_str(&replace.string);
                last_position = replace.end_position as usize;
            }
        }
    }
    result.push_str(&script[last_position..]);
    return result;
}

use rand::seq::SliceRandom;

use super::{js_utils::search_json, utils_swc::parse_with_swc};

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
            let alphabet: [char; 53] = [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'v', 'u', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F',
                'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'V', 'U',
                'W', 'X', 'Y', 'Z', '$',
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

pub fn append_v_to_vars_in_html(input: &str, variables: &Vec<String>) -> (String, Vec<String>) {
    let parsed = parse_with_swc(&input.to_string());

    let parsed_json = serde_json::to_value(&parsed).unwrap();

    let (positions, _, depending_vars) =
        search_json(&parsed_json, &input.to_string(), &variables, None, None);

    let modified_string = add_or_remove_strings_to_script(positions, &input.to_string());

    (modified_string, depending_vars)
}
