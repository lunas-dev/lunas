mod generate_js;
mod structs;
mod transformers;
use crate::generate_js::generate_js_from_blocks;
use blve_parser::parse_blve_file;

fn main() {
    // "Hello, world!".to_string()

    let a = r#"
html:
  <h1 id='abc'>Hello Blve!</h1>
  <div >${count}</div>
  <button @click="increment">+1</button>
  <button @click="clear">${interval==null?"start":"clear"}</button>
script:
  let count = 0
  function increment(){
    count++
    console.log(count)
  }
  function clear(){
    if(interval){
      clearInterval(interval)
      interval = null
    }else{
      interval = setInterval(increment, 2000)
    }
  }
  let interval = setInterval(increment, 2000)
style:
  h1 {
    color: blue;
  }
  * {
    font-family: 'Noto Sans', sans-serif;
  }

"#;

    let mut b = parse_blve_file(a).unwrap();
    let code = generate_js_from_blocks(&mut b);
    println!("{}", code.0);
}
