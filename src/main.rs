
mod parse;
mod translate;

fn main() {

    let regex = "[A-F0-9]{8}(?:-[A-F0-9]{4}){3}-[A-F0-9]{12}";

    regex::Regex::new(regex).unwrap();

    let ast = parse::NativeRegexAST::from(regex.as_bytes());

    println!("----- AST -----");

    ast.tree();

    println!("----- CAPTURES {} -----", ast.get_captures());

    println!("----- SOURCE ----- \n{}", translate::translate_ast_wrapper(regex, "testing"));

}
