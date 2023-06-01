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
  <button @click="increment">${count}</button>
  <div>${count2}</div>
  <button @click="increment2">count2++</button>
  <div>${count2+count}</div>
script:
  let count = 0
  function increment(){
    count++
  }
  let count2 = 10
  function increment2(){
    count2++
  }
"#;

    let mut b = parse_blve_file(a).unwrap();
    let code = generate_js_from_blocks(&mut b);
    println!("{}", code);
}
