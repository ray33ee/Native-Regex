
use std::ops::RangeInclusive;

#[derive(Debug)]
enum RepeaterType {
    ExactlyOnce, //No repetition
    ZeroAndOne, //?
    ZeroAndAbove, //*
    OneAndAbove, //+
    ExactlyN(usize), //{N}
    Range(usize, usize) //{N, M}
}

#[derive(Debug)]
struct Repeater {
    repeater: RepeaterType,
    length: usize
}

#[derive(Debug)]
enum GroupType {
    Capturing, //()
    NonCapturing //(?:)
}

#[derive(Debug)]
enum AnchorType {
    Start, //^
    End //$
}

#[derive(Debug)]
pub struct CharacterSet {
    set: Vec<RangeInclusive<usize>>
}

#[derive(Debug)]
enum Token<'a> {
    CharacterClass(CharacterSet, RepeaterType), //Can be a traditional character class, [] or a shorthand character class
    Anchor(AnchorType),
    LiteralSingle(u8, RepeaterType),
    LiteralList(& 'a [u8]),
    Group(NativeRegexAST<'a>, RepeaterType, GroupType),
}

#[derive(Debug)]
pub struct NativeRegexAST<'a> {
    tokens: Vec<Token<'a>>
}

impl From<& [u8]> for CharacterSet {
    fn from(mut slice: & [u8]) -> Self {

        let mut set = Vec::new();

        //Look for a '^' and chop it off
        let negate = if slice[0] == '^' as u8 {
            slice = &slice[1..];
            true
        } else {
            false
        };

        //Similarly if we have leading or trailing '-' add them and chop the string
        if slice[0] == '-' as u8 {
            slice = &slice[1..];
            set.push(RangeInclusive::new('-' as usize, '-' as usize));
        }

        if slice[slice.len() - 1] == '-' as u8 {
            slice = &slice[..slice.len()-1];
            set.push(RangeInclusive::new('-' as usize, '-' as usize));
        }

        let mut index = 0;

        while index < slice.len() {

            let mut counter = 0;

            let mut ch = '\0' as usize;

            //Get current token
            if slice[index] == '\\' as u8 {

                if slice[index+1] == 'd' as u8 {

                    index += 2;
                    continue;
                } else if slice[index+1] == 's' as u8 {

                    index += 2;
                    continue;
                } else if slice[index+1] == 'w' as u8 {

                    index += 2;
                    continue;
                } else if slice[index+1] == 'l' as u8 {

                    index += 2;
                    continue;
                } else if slice[index+1] == 'u' as u8 {

                    index += 2;
                    continue;
                } else {
                    ch = slice[index+1] as usize;
                    counter += 2;
                }



            } else {
                ch = slice[index] as usize;
                counter += 1;
            }

            if index + counter < slice.len() {

                //Get next token. If it's a '-' we have a range of characters
                if slice[index + counter] == '-' as u8 {
                    set.push(RangeInclusive::new(ch as usize, slice[index + counter + 1] as usize));
                    index += counter + 2;
                } else {
                    set.push(RangeInclusive::new(ch, ch));
                    index += counter;
                }
            } else {
                set.push(RangeInclusive::new(ch, ch));
                index += counter;
            }

        }

        CharacterSet {
            set
        }
    }
}

impl From<& [u8]> for Repeater{
    fn from(slice: & [u8]) -> Self {


        let mut repeater = RepeaterType::ExactlyOnce;
        let mut length = 0;

        if !slice.is_empty() {
            let first_char = slice[0];

            if first_char == '?' as u8 {
                repeater = RepeaterType::ZeroAndOne;
                length = 1;
            } else if first_char == '+' as u8 {
                repeater = RepeaterType::OneAndAbove;
                length = 1;
            } else if first_char == '*' as u8 {
                repeater = RepeaterType::ZeroAndAbove;
                length = 1;
            } else if first_char == '{' as u8 {

            } else {
                //repeater = RepeaterType::ExactlyOnce;
                //length = 0;
            }
        }

        Repeater {
            repeater,
            length,
        }
    }
}

impl Token<'_> {
    fn from(slice: & [u8]) -> (Self, & [u8]) {

        let first_char = slice[0];

        if first_char == '[' as u8 {

            let mut close_index = 1;

            for ch in &slice[1..] {
                if *ch == ']' as u8 {
                    break;
                }
                close_index += 1;
            }

            let repeater = Repeater::from(&slice[close_index+1..]);

            (Token::CharacterClass(CharacterSet::from(&slice[1..close_index]), repeater.repeater), &slice[close_index+repeater.length+1..])

        } else if first_char == '$' as u8 {
            (Token::Anchor(AnchorType::End), &slice[1..])
        } else if first_char == '^' as u8 {
            (Token::Anchor(AnchorType::Start), &slice[1..])
        } else if first_char == '\\' as u8 ||
            first_char >= 'a' as u8 &&  first_char <= 'z' as u8 ||
            first_char >= 'A' as u8 &&  first_char <= 'Z' as u8 ||
            first_char >= '0' as u8 &&  first_char <= '9' as u8 {

            //Keep searching until we find a character that's not a literal character
            //If the terminating character is a repeater then stop before the last character and ship as LiteralList
            //Otherwise ship as a LiteralSingle
            //If we have a single character with no repeater, ship as LiteralSingle.
            //This one might take a bit more thought...
            //Don't forget escaped characters like \+
            //Also \\ will match with shorthand character classes too, so yehhh...

            if first_char == '\\' as u8 {
                let repeater = Repeater::from(&slice[2..]);
                (Token::LiteralSingle(slice[1], repeater.repeater), &slice[repeater.length+2..])
            } else {
                let repeater = Repeater::from(&slice[1..]);
                (Token::LiteralSingle(first_char, repeater.repeater), &slice[repeater.length+1..])
            }

        } else if first_char == '(' as u8 {
            let mut nest_depth = 1;
            let mut close_index = 1;

            for ch in &slice[1..] {
                if *ch == '(' as u8 {
                    nest_depth += 1;
                } else if *ch == ')' as u8 {
                    nest_depth -= 1;

                    if nest_depth == 0 {
                        break;
                    }
                }
                close_index += 1;
            }

            let repeater = Repeater::from(&slice[close_index+1..]);

            if slice[1] == '?' as u8 && slice[2] == ':' as u8 {
                (Token::Group(NativeRegexAST::from(&slice[3..close_index]),
                             repeater.repeater,
                             GroupType::NonCapturing), &slice[close_index+repeater.length+1..])
            } else {
                (Token::Group(NativeRegexAST::from(&slice[1..close_index]),
                             repeater.repeater,
                             GroupType::Capturing), &slice[close_index+repeater.length+1..])
            }



        } else {
            //Match as literal?
            panic!("Not yet implemented - Match anything else");
        }

    }
}


impl From<& [u8]> for NativeRegexAST<'_> {
    fn from(regex: & [u8]) -> Self {

        let mut tokens = Vec::new();

        let mut remainder = regex;

        while !remainder.is_empty() {

            let (tok, rem) = Token::from(remainder);

            remainder = rem;

            tokens.push(tok);


        }

        NativeRegexAST {
            tokens
        }
    }
}

