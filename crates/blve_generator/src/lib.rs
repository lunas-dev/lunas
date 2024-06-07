mod generate_js;
mod generate_statements;
mod orig_html_struct;
mod structs;
mod transformers;
use blve_parser::DetailedBlock;
use generate_js::generate_js_from_blocks;
#[macro_use]
extern crate lazy_static;

pub fn blve_compile_from_block(
    b: &DetailedBlock,
    no_export: Option<bool>,
    export_name: Option<String>,
    runtime_path: Option<String>,
) -> Result<(String, Option<String>), String> {
    let compiled_code = generate_js_from_blocks(b, no_export, export_name, runtime_path);
    compiled_code
}
