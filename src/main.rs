
use std::time::Instant;

mod macros;
mod tests;
mod example;
mod parse;

//Hard coded regex example. We use the following regex:

fn main() {

    let regex = "\\+";

    regex::Regex::new(regex).unwrap();

    let ast = parse::NativeRegexAST::from(regex.as_bytes());

    println!("Tree: {:?}", ast);

    /*let set = parse::CharacterSet::from("-[0e-9-g]-".as_bytes());

    println!("set: {:?}", set);*/

    //let re = regex::Regex::new("[-]\\").unwrap();

    /*let text1 = "(sdldlsdf  m podfpodk";

    let start = Instant::now();
    let result = example::reg_double_nested(text1);
    let native_regex_duration = start.elapsed();

    println!("Native Regex result:  {:?}", result);
    println!("Native Regex elapsed: {:?}", native_regex_duration);

    let start = Instant::now();
    let re = regex::Regex::new("[0-9[a-z]]").unwrap();
    let compile_duration = start.elapsed();

    let start = Instant::now();
    let mach = re.captures(text1);
    let matching_duration = start.elapsed();

    println!("Vanilla Regex result: {:?}", mach);
    println!("Vanilla compile elapsed: {:?}", compile_duration);
    println!("Vanilla match elapsed: {:?}", matching_duration);

    */




}
