mod generate_js;
mod structs;
mod transformers;
use crate::generate_js::generate_js_from_blocks;
use blve_parser::parse_blve_file;

fn main() {
    // "Hello, world!".to_string()

    let a = r#"
html:
    <h1>Hello Blve!</h1>
    <button id="abc" @click="increment">${count+count} count ${count} ${ count + count }</button>
script:
    let count = 0
    let count2 = 0
    let count3 = 0
    function increment(){
        count++
        count++
        count++
        count++
        count = count + 200
    }
"#;

    let mut b = parse_blve_file(a).unwrap();
    let code = generate_js_from_blocks(&mut b);
    println!("{}", code);
}
