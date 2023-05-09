mod parse2;
mod parser1;
mod parsers;
mod structs;
use parse2::parse2;
use parser1::parse1;

fn main() {
    let a = parse1(
        r#"


@input message: string = "hey"
@input optional:string?
@use MyComponent from './components/my-component.blv'

html:
    <div>

    <h1>
    Hello World
    </h1>
    </div>
@input finalInput:String = "final input"


"#,
    );

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

    // let mut list_of_language_block = Vec::new();
    // match a {
    //     Ok((_, b)) => {
    //         // pretty print the parsed items
    //         // println!("{:?}", b)}
    //         for i in &b {
    //             println!("{:?}", i)
    //         }

    //         for i in b {
    //             match i {
    //                 LanguageBlock(lb) => {
    //                     list_of_language_block.push(lb);
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    //     Err(e) => println!("{:?}", e),
    // }

    // let mut lang_map = HashMap::new();

    // for item in list_of_language_block {
    //     let lang_name = item.language_name;
    //     if lang_name != "html" && lang_name != "script" {
    //         println!("Error: Language name not recognized");
    //         break;
    //     }
    //     let lang_content = item.content;
    //     if lang_map.contains_key(&lang_name) {
    //         println!("Error: Language already declared");
    //         break;
    //     }
    //     lang_map.insert(lang_name, lang_content);
    // }
}

/* @state(key:"hey") myState:AppState
@input message = "hey"
@input optional:string?
@use MyComponent from './components/my-component.blv'

@input(param:hey) optional?
html:
    <div>

    <h1>
    Hello World
    </h1>
    </div>
@input finalInput = "final input" */
