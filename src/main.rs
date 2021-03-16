
use clap::{Arg, App, crate_version, crate_authors};
use std::fs::OpenOptions;
use std::io::Write;

mod parse;
mod rust_translate;


fn main() -> Result<(), String> {

    let matches = App::new("Native Regex")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Tool for converting regexes into source code")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .help("File to output source code to")
            .required(true)
            .validator(|file_name| {
                if file_name.is_empty() {
                    Err(String::from("Please enter a valid file name"))
                } else {
                    Ok(())
                }
            })
            .takes_value(true))
        .arg(Arg::with_name("regex")
            .short("r")
            .long("regex")
            .help("The regex to convert into source")
            .validator(|regex| {
                match regex::Regex::new(&regex) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("'{}' is not a valid regex - {}", regex, e))
                }
            })
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("function name")
            .short("n")
            .long("name")
            .help("Name of the function in output source")
            .required(true)
            .takes_value(true)
            .validator(|_function_name| {
                Ok(())
            }))
        .arg(Arg::with_name("language")
            .short("l")
            .long("language")
            .help("The programming language of the output source")
            .required(true)
            .takes_value(true)
            .possible_values(&["Rust"]))
        .arg(Arg::with_name("verbosity")
            .short("v")
            .long("verbose")
            .help("Show extra information")
            .required(false)
            .takes_value(false))
        .get_matches();

    let regex = matches.value_of("regex").unwrap();

    let function_name = matches.value_of("function name").unwrap();

    let verbosity = matches.is_present("verbosity");

    let file_name = matches.value_of("file").unwrap();

    let ast = parse::NativeRegexAST::from(regex.as_bytes());

    if verbosity {
        println!("----- AST -----");

        ast.tree();

        println!("----- CAPTURES {} -----", ast.get_captures());
    }

    let translation_result = match matches.value_of("language").unwrap() {
        "Rust" => {
            rust_translate::translate(regex, function_name)
        }
        _ => { unreachable!() }
    };

    match translation_result {
        Ok(code) => {

            if verbosity {
                println!("----- SOURCE ----- \n{}", code);
                println!("----- END -----");
            }

            let file_result = OpenOptions::new()
                .read(false)
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_name);

            match file_result {
                Ok(mut handle) => {
                    match handle.write_all(code.as_bytes()) {
                        Ok(_) => {
                            Ok(())
                        }
                        Err(e) => {
                            Err(format!("Could not save source code to file '{}' - {}", file_name, e))
                        }
                    }
                }
                Err(e) => {
                    Err(format!("Could not save source code to file '{}' - {}", file_name, e))
                }
            }
        }
        Err(e) => {
            Err(format!("Could not translate regex - {}", e))
        }
    }
}
