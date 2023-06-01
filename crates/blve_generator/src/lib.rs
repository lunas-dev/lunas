mod generate_js;
mod structs;
mod transformers;
use blve_parser::DetailedBlock;
use generate_js::generate_js_from_blocks;

pub fn blve_compile_from_block(b: &mut DetailedBlock) -> (String, Option<String>) {
    let compiled_code = generate_js_from_blocks(b);
    compiled_code
}
