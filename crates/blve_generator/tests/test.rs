#[cfg(test)]
use std::fs;
use std::path::PathBuf;

use blve_parser::parse_blve_file;
use pretty_assertions::assert_eq;
extern crate blve_generator;
use blve_generator::blve_compile_from_block;

#[test]
fn test_case_files() {
    std::env::set_var("BLVE_TEST", "true");
    let test_dir = "tests/cases";
    let input_extension = "blv";
    let output_extension = "js";

    let test_files = fs::read_dir(test_dir).expect("Failed to read test directory");

    for test_file in test_files {
        let file = test_file.expect("Failed to read test file");
        let file_path = file.path();
        let file_extension = file_path.extension();

        if let Some(extension) = file_extension {
            if extension == input_extension {
                let input_content =
                    fs::read_to_string(&file_path).expect("Failed to read input file");

                let output_file_name = file_path.file_stem().unwrap().to_string_lossy();
                let output_file_name_with_extension =
                    format!("{}.{}", output_file_name, output_extension);
                let output_path = PathBuf::from(test_dir).join(output_file_name_with_extension);

                let test_input =
                    fs::read_to_string(&output_path).expect("Failed to read output file");

                let b = parse_blve_file(input_content.as_str()).unwrap();
                let compiled_input = blve_compile_from_block(&b, None, None, None).unwrap().0;

                assert_eq!(
                    test_input.as_str(),
                    compiled_input.as_str(),
                    "Test case {:?} failed",
                    file_path
                );
            }
        }
    }
}
