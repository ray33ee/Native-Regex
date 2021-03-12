
use std::time::Instant;

mod macros;
mod tests;
mod example;
mod parse;
mod translate;

//Hard coded regex example. We use the following regex:


fn main() {

    let regex = "[-+]?\\b[0-9]*\\.?[0-9]+\\b";

    let regex = "\\b(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\\.(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\\b";

    let regex = "[0-9]h(e(fff)l)o(he)";

    regex::Regex::new(regex).unwrap();

    let ast = parse::NativeRegexAST::from(regex.as_bytes());

    //ast.tree();

    //println!("Capture count: {}", ast.get_captures());

    let mut capture_index = 0usize;

    println!("{}", translate::translate_ast(&ast, true, & mut capture_index, false));

    /*let set = parse::CharacterSet::from("-[0e-9-g]-".as_bytes());

    println!("set: {:?}", set);*/

    //let re = regex::Regex::new("[-]\\").unwrap();

    /*let text1 = "(sdldlsdf 8423453454921e podfpodk";

    let start = Instant::now();
    let result = example::reg_n_above(text1);
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



*/


}
