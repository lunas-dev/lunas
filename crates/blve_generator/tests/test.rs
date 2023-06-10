#[cfg(test)]
use std::fs;
use std::path::PathBuf;

use blve_parser::parse_blve_file;
use pretty_assertions::assert_eq;
extern crate blve_generator;
use blve_generator::blve_compile_from_block;

#[test]
fn test_case_files() {
    let test_dir = "tests/cases";
    let input_extension = "in";
    let output_extension = "out";

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

                let output_content =
                    fs::read_to_string(&output_path).expect("Failed to read output file");

                let b = parse_blve_file(input_content.as_str()).unwrap();
                let compiled_input = blve_compile_from_block(&b).0;

                assert_eq!(
                    remove_random_string(compiled_input.as_str()),
                    remove_random_string(output_content.as_str()),
                    "Test case {:?} failed",
                    file_path
                );
            }
        }
    }
}

fn remove_random_string(input: &str) -> String {
    let re = regex::Regex::new(r"[a-zA-Z]{10}").unwrap();
    re.replace_all(input, "").into_owned()
}
