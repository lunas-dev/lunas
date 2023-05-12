mod parse2;
mod parser1;
mod parsers;
mod structs;
use parse2::parse2;
use parser1::parse1;

fn main() {
    // load ./sample/1.blv
    let input = std::fs::read_to_string("./sample/1.blv").unwrap();

    let a = parse1(input.as_str());

    match a {
        Ok((_, b)) => {
            let c = parse2(b);
            match c {
                Ok(d) => {
                    println!("{:#?}", d);
                    return;
                }
                Err(e) => {
                    println!("{:?}", e);
                    return;
                }
            }
        }
        Err(_) => println!("err"),
    };
}

