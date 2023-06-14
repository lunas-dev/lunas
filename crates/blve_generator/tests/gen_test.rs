use std::fs;

use blve_generator::blve_compile_from_block;
use blve_parser::parse_blve_file;

fn main() -> Result<(), String> {
    // get the file path from the command line
    let args: Vec<String> = std::env::args().collect();
    let a = fs::read_to_string(format!("./tests/cases/{}.in", args[1])).unwrap();
    let b = parse_blve_file(a.as_str()).unwrap();
    let code = blve_compile_from_block(&b);
    println!("{}", code.0);
    fs::write(format!("./tests/cases/{}.out", args[1]), code.0).unwrap();
    Ok(())
}
