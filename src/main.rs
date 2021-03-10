
use std::time::Instant;
use core::num::FpCategory::Infinite;
use regex::internal::Inst;

mod macros;
mod tests;
mod example;

//Hard coded regex example. We use the following regex:

fn main() {

    //er(([a-f][fd])(hell))[4-6]g+

    let text1 = "iodafid De erddhellhellhellhellhellhe5gggg sdwe-295";

    let start = Instant::now();
    let result = example::reg_nested_captures(text1);
    let native_regex_duration = start.elapsed();

    println!("Native Regex result:  {:?}", result);
    println!("Native Regex elapsed: {:?}", native_regex_duration);

    let start = Instant::now();
    let re = regex::Regex::new("(hell)+").unwrap();
    let compile_duration = start.elapsed();

    let start = Instant::now();
    let mach = re.captures(text1);
    let matching_duration = start.elapsed();

    println!("Vanilla Regex result: {:?}", mach);
    println!("Vanilla compile elapsed: {:?}", compile_duration);
    println!("Vanilla match elapsed: {:?}", matching_duration);



    use regex::Regex;




}
