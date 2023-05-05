mod parser;
mod parsers;
mod structs;
use parser::parse;

fn main() {
    let a = parse(
        r#"


@input(arg1:hello) "hey"
@input "hey"
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
