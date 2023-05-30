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
                    match &d.detailed_language_blocks.js {
                        Some(js) => {
                            let js = js.as_js_module();
                            match js {
                                Some(js) => {
                                    // js.source_type = Some(rome_js_syntax::SourceType::Module);
                                    println!("{:#?}", js);

                                    // js.

                                    let a = js.to_string();
                                    println!("{}", a);
                                }
                                None => {}
                            }
                            // println!("{:#?}", js);
                        }
                        None => {
                            // Err("No js block")
                        }
                    }
                    println!("{:#?}", &d);
                    let id = nanoid!();
                    let filename = format!("{}_{}.blvestruct", filenum, id);
                    std::fs::write(filename, format!("{:#?}", d.detailed_language_blocks.js))
                        .unwrap();
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
