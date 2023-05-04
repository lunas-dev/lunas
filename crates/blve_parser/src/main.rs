mod parser;
use parser::parse;

fn main() {
    let a = parse(
        r#"@input(arg1:hello) "hey"
html:
    <div>

    <h1>
    Hello World
    </h1>
    </div>"#,
    );
    match a {
        Ok((_, b)) => println!("{:?}", b),
        Err(e) => println!("{:?}", e),
    }
}
