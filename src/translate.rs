use crate::parse::*;
use std::ops::RangeInclusive;

fn bounds_check(n: usize) -> String {
    format!("if index + counter + ({} - 1) > text.len() {{ index += 1; continue 'main; }}", n)
}

fn envelope(inner_code: String, repeater: &RepeaterType, nomatch: & str) -> String {

    match repeater {
        RepeaterType::ExactlyOnce => {
            inner_code
        },
        RepeaterType::ZeroAndOne => {
        format!("{{
    let mut match_count = 0;

    for _ in &text[index + counter..] {{
        {}

        match_count += 1;

        if match_count == 1 {{
            break;
        }}
    }}

}}\n\n", inner_code)
        },
        RepeaterType::OneAndAbove => {
            format!("{{
    let mut found = false;

    for _ in &text[index + counter..] {{
        {}
        found = true;
    }}

    if !found {{
        {}
    }}
}}\n\n", inner_code, nomatch)
        },
        RepeaterType::ZeroAndAbove => {
            format!("for _ in &text[index + counter..] {{
    {}
}}\n\n", inner_code)
        },
        RepeaterType::ExactlyN(n) => {
            format!("{{
    let mut match_count = 0;

    for _ in &text[index + counter..] {{
        {}

        match_count += 1;

        if match_count == {} {{
            break;
        }}
    }}

    if match_count < {} {{
        {}
    }}
}}\n\n", inner_code, n, n, nomatch)
        },
        RepeaterType::Range(n, m) => {
            format!("{{
    let mut match_count = 0;

    for _ in &text[index + counter..] {{
        {}

        match_count += 1;

        if match_count == {} {{
            break;
        }}
    }}

    if match_count < {} {{
        {}
    }}
}}\n\n", inner_code, m, n, nomatch)
        },
        RepeaterType::NAndAbove(n) => {
            format!("{{
    let mut match_count = 0;

    for _ in &text[index + counter..] {{
        {}

        match_count += 1;
    }}

    if match_count < {} {{
        {}
    }}
}}\n\n", inner_code, n, nomatch)
        }
    }
}

fn get_no_match(inforloop: bool) -> & 'static str {
    if inforloop { "break;" } else { "index += 1; continue;" }
}

fn range_to_condition(range: & RangeInclusive<u8>) -> String {
    if range.start() == range.end() {
        format!("text[index+counter] == '{}' as u8", *range.start() as char)
    } else {
        format!("text[index+counter] >= '{}' as u8 && text[index+counter] <= '{}' as u8", *range.start() as char, *range.end() as char)
    }
}

fn token_translate(token: & Token, capture_index: & mut usize, inforloop: bool) -> String {

    let nomatch = get_no_match(inforloop);

    match token {
        Token::LiteralSingle(character, repeater) => {
            let nomatch = get_no_match(*repeater != RepeaterType::ExactlyOnce || inforloop);
            envelope(format!("{}\n\nif text[index + counter] != '{}' as u8 {{ {} }}\n\ncounter += 1;\n\n", bounds_check(1), *character as char, nomatch), repeater, nomatch)
        },
        Token::LiteralList(list) => {
            let mut conditions = format!("text[index + counter] == '{}' as u8", list[0] as char);
            for i in 1..list.len() {
                conditions = format!("{} && text[index + counter + {}] == '{}' as u8", conditions, i, list[i] as char)
            }
            format!("{}\n\nif !({}) {{ {} }}\n\ncounter += {};\n\n", bounds_check(list.len()), conditions, nomatch, list.len())
        },
        Token::Anchor(anchor) => match anchor {
            AnchorType::Start => {
                String::from("if index != 0 { return None; }")
            },
            AnchorType::End => {
                String::from("if index != text.len()-1 { index += 1; continue; }")
            },
            AnchorType::WordBorder => {
                String::from("panic!(\"WORD BORDER NOT SUPPORTED\");")
            }
        },
        Token::Alternation => {
            String::from("panic!(\"ALTERNATION DEFINITELY NOT SUPPORTED\");")
        },
        Token::CharacterClass(set, repeater) => {
            let outernomatch = get_no_match(inforloop);

            let withinnomatch = get_no_match(*repeater != RepeaterType::ExactlyOnce || inforloop);

            let mut ranges = range_to_condition(&set.set[0]);

            for i in 1..set.set.len() {
                ranges = format!("{} || {}", ranges, range_to_condition(&set.set[i]))
            }

            envelope(format!("{}\n\nif {}({}) {{ {} }}\n\ncounter += 1;\n\n", bounds_check(1), if set.inverted { "" } else { "!" }, ranges, withinnomatch), repeater, outernomatch)
        },
        Token::Group(ast, repeater, group) => {

            //If the parent ast node is in a for loop, or the repeater of this capture group is a for loop
            let isinforloop = *repeater != RepeaterType::ExactlyOnce || inforloop;

            let code = match group {
                GroupType::Capturing => {
                    let capture_start = format!("let capture_{}_start = index+counter;\n\n", *capture_index);
                    let capture_end = format!("captures[{}] = Some((index + counter, &str_text[capture_{}_start..index + counter]));\n\n", *capture_index, *capture_index);

                    *capture_index += 1;

                    let inner_code = translate_ast(ast, capture_index, isinforloop);

                    envelope(format!("{{\n\n{}{}{}}}\n\n", capture_start, inner_code, capture_end), repeater, nomatch)
                },
                GroupType::NonCapturing => {
                    let inner_code = translate_ast(ast, capture_index, isinforloop);

                    envelope(format!("{{\n\n{}}}\n\n", inner_code), repeater, nomatch)
                }
            };


            code
        }
    }
}

fn translate_ast(ast: & NativeRegexAST, capture_index: & mut usize, inforloop: bool) -> String {
    let mut code = String::new();

    for token in ast.tokens.iter() {
        code = format!("{}{}", code, token_translate(token, capture_index, inforloop));
    }

    code
}

pub fn translate_ast_wrapper(regex: & str, function_name: & str) -> String {

    //Add the base code, including capture array/vector and custom function name

    let mut capture_index = 1;


    let ast = crate::parse::NativeRegexAST::from(regex.as_bytes());

    format!("// Hard coded function to match regex '{}'
pub fn {}(str_text: &str) -> Option<Vec<Option<(usize, & str)>>> {{
    let text = str_text.as_bytes();

    let mut index = 0;

    let mut captures = vec![None; {}];

    'main: while index < text.len() {{

        //Start counter
        let mut counter = 0;

        let capture_0_start = index + counter;

        {}

        captures[0] = Some((index + counter, &str_text[capture_0_start..index+counter]));

        return Some(captures);
    }}


    None
}}", regex, function_name, ast.get_captures(), translate_ast(&ast, & mut capture_index, false))
}

