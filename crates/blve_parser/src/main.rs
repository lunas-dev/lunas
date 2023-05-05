mod parser;
mod parsers;
mod structs;
use parser::parse;

fn main() {
    let a = parse(
        r#"


@state(key:"hey") myState:AppState
@input message = "hey"
@input optional:string?

@input(param:hey) optional?
html:
    <div>

    <h1>
    Hello World
    </h1>
    </div>
@input "last input"


"#,
    );
    match a {
        Ok((_, b)) => {
            // pretty print the parsed items
            // println!("{:?}", b)}
            for i in b {
                println!("{:?}", i)
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
