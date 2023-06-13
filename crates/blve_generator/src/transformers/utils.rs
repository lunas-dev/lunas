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
    let alphabet: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'v', 'u', 'w', 'x', 'y', 'z',
    ];
    nanoid::nanoid!(10, &alphabet)
}
