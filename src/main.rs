
mod parse;
mod translate;

// Hard coded function to match regex '[^a]+'
pub fn test_function(str_text: &str) -> Option<Vec<Option<(usize, & str)>>> {
    let text = str_text.as_bytes();

    let mut index = 0;

    let mut captures = vec![None; 1];

    'main: while index < text.len() {

        //Start counter
        let mut counter = 0;

        let capture_0_start = index + counter;

        {
            let mut found = false;

            for _ in &text[index + counter..] {
                if index + counter + (1 - 1) > text.len() { index += 1; continue 'main; }

                if text[index+counter] == 97 { break; }

                counter += 1;


                found = true;
            }

            if !found {
                index += 1; continue;
            }
        }



        captures[0] = Some((index + counter, &str_text[capture_0_start..index+counter]));

        return Some(captures);
    }


    None
}

fn main() {

    let regex = "[^a]+";

    regex::Regex::new(regex).unwrap();

    let ast = parse::NativeRegexAST::from(regex.as_bytes());

    println!("----- AST -----");

    ast.tree();

    println!("----- CAPTURES {} -----", ast.get_captures());

    println!("----- SOURCE ----- \n{}", translate::translate_ast_wrapper(regex, "test_text").unwrap());

    let test_text = "siddiffghpogjpohasd";

    println!("----- TESTING ----- \n{:?}", test_function(test_text));

}
