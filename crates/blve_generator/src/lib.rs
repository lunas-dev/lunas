mod generate_js;
mod structs;
mod transformers;
mod html_with_relation;
use blve_parser::DetailedBlock;
use generate_js::generate_js_from_blocks;

pub fn blve_compile_from_block(b: &DetailedBlock) -> Result<(String, Option<String>), String> {
    let compiled_code = generate_js_from_blocks(b);
    compiled_code
}
