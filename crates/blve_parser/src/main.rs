mod parse2;
mod parser1;
mod parsers;
mod structs;
use nanoid::nanoid;
use parse2::parse2;
use parser1::parse1;

fn main() {
    let filenum = "3";
    let filepath = format!("./sample/{}.blv", filenum);
    let input = std::fs::read_to_string(filepath).unwrap();

    let a = parse1(input.as_str());

    match a {
        Ok((_, b)) => {
            let c = parse2(b);
            match c {
                Ok(d) => {
                    println!("{:#?}", d);
                    let id = nanoid!();
                    // gen random string for filename
                    let filename = format!("{}_{}.blvestruct", filenum, id);
                    std::fs::write(filename, format!("{:#?}", d)).unwrap();
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
