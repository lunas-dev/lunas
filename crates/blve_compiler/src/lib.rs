use blve_generator::blve_compile_from_block;
use blve_parser::parse_blve_file;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile(blve_code: String) -> Result<String, String> {
    let mut blocks = match parse_blve_file(&blve_code) {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }?;
    let code = blve_compile_from_block(&mut blocks);
    Ok(code)
}
