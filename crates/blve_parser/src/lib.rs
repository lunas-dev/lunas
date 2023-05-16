/* mod parse2;
mod parser1;
mod parsers;
mod structs;
use parse2::parse2;
use parser1::parse1;

fn parse(input:String) -> Result<structs::detailed_blocks::DetailedBlock, &str> {
    let a = parse1(input.as_str());

    match a {
        Ok((_, b)) => {
            let c = parse2(b);
            match c {
                Ok(d) => {
                    println!("{:#?}", d);
                    let k = "{:#?}";
                    // save k as ${random}.blvestruct file


                    d
                }
                Err(e) => {
                    println!("{:?}", e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
          Err(e.to_string().as_str())
        },
    };
}
 */