use crate::parse::*;
use crate::parse::Token::LiteralSingle;
use std::ops::RangeInclusive;

fn bounds_check(n: usize, nomatch: & 'static str) -> String {
    format!("if index + counter + ({} - 1) > text.len() {{ {} }}", n, nomatch)
}

fn envelope(inner_code: String, repeater: &RepeaterType) -> String {


    inner_code
}

fn range_to_condition(range: & RangeInclusive<u8>) -> String {
    if range.start() == range.end() {
        format!("text[index+counter] == {}", range.start())
    } else {
        format!("text[index+counter] >= {} && text[index+counter] <= {}", range.start(), range.end())
    }
}

fn token_translate(token: & Token, nomatch: & 'static str, capture_index: & mut usize, inforloop: bool) -> String {

    let nomatch = if inforloop { "break;" } else { "index += 1; continue;" };

    match token {
        Token::LiteralSingle(character, repeater) => {
            envelope(format!("{}\n\nif text[index + counter] == {} {{ {} }}\n\ncounter += 1;\n\n", bounds_check(1, nomatch), character, nomatch), repeater)
        },
        Token::LiteralList(list) => {
            let mut conditions = format!("text[index + counter] == {}", list[0]);
            for i in 1..list.len() {
                conditions = format!("{} && text[index + counter + {}] == {}", conditions, i, list[i])
            }
            format!("{}\n\nif !({}) {{ {} }}\n\ncounter += {};\n\n", bounds_check(list.len(), nomatch), conditions, nomatch, list.len())
        },
        Token::Anchor(anchor) => match anchor {
            AnchorType::Start => {
                String::from("if index != 0 { return None; }")
            },
            AnchorType::End => {
                String::from("if index != text.len()-1 { index += 1; continue; }")
            },
            AnchorType::WordBorder => {
                String::from("panic!(\"WORD BORDER NOT SUPPORTED\")")
            }
        },
        Token::Alternation => {
            String::from("panic!(\"ALTERNATION DEFINITELY NOT SUPPORTED\")")
        },
        Token::CharacterClass(set, repeater) => {
            let mut ranges = range_to_condition(&set.set[0]);
            for i in 1..set.set.len() {
                ranges = format!("{} || {}", ranges, range_to_condition(&set.set[i]))
            }
            envelope(format!("{}\n\nif {}({}) {{ {} }}\n\ncounter += 1;\n\n", bounds_check(1, nomatch), if set.inverted { "" } else { "!" }, ranges, nomatch), repeater)
        },
        Token::Group(ast, repeater, group) => {

            let code = match group {
                GroupType::Capturing => {
                    let capture_start = format!("let capture_{}_start = index+counter;\n\n", *capture_index);
                    let capture_end = format!("captures[{}] = Some(index + counter, &text[capture_{}_start..index + counter]);\n\n", *capture_index, *capture_index);

                    *capture_index += 1;

                    let inner_code = translate_ast(ast, false, capture_index, false);



                    envelope(format!("{{\n\n{}{}{}}}\n\n", capture_start, inner_code, capture_end), repeater)
                },
                GroupType::NonCapturing => {
                    let inner_code = translate_ast(ast, false, capture_index, false);

                    envelope(format!("{{\n\n{}}}\n\n", inner_code), repeater)
                }
            };


            code
        }
        _ => {
            String::new()
        }
    }
}

pub fn translate_ast(ast: & NativeRegexAST, is_top: bool, capture_index: & mut usize, inforloop: bool) -> String {
    let mut code = String::new();

    let no_match_break = if is_top { "index += 1; continue;"} else { "break;" };

    for token in ast.tokens.iter() {
        code = format!("{}{}", code, token_translate(token, no_match_break, capture_index, inforloop));
    }

    code
}

