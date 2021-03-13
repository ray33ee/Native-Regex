
use std::time::Instant;

mod parse;
mod translate;
mod code;

//Hard coded regex example. We use the following regex:


fn main() {

    let regex = "[-+]?\\b[0-9]*\\.?[0-9]+\\b";

    let regex = "\\b(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\\b";

    let regex = "([A-Z][a-z]*)([0-9]*)";

    regex::Regex::new(regex).unwrap();

    let ast = parse::NativeRegexAST::from(regex.as_bytes());

    //ast.tree();

    //println!("Capture count: {}", ast.get_captures());

    let mut capture_index = 0usize;

    println!("{}", translate::translate_ast_wrapper(regex, "testing"));

    /*let set = parse::CharacterSet::from("-[0e-9-g]-".as_bytes());

    println!("set: {:?}", set);*/

    //let re = regex::Regex::new("[-]\\").unwrap();
    //(?:[0-9][A-Z]){3-6}[a-z]
    let text1 = "pl[pldff 4[p[dafpdf";

    let start = Instant::now();
    let result = code::testing(text1);
    let native_regex_duration = start.elapsed();

    println!("Native Regex result:  {:?}", result);
    println!("Native Regex elapsed: {:?}", native_regex_duration);

    let start = Instant::now();
    let re = regex::Regex::new("([0-9]*)([0-9])").unwrap();
    let compile_duration = start.elapsed();

    let start = Instant::now();
    let mach = re.captures(text1);
    let matching_duration = start.elapsed();

    println!("Vanilla Regex result: {:?}", mach);
    println!("Vanilla compile elapsed: {:?}", compile_duration);
    println!("Vanilla match elapsed: {:?}", matching_duration);






}
