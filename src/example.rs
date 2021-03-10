//Set of example, hard coded functions based on corresponding regexes

//([A-Z][a-z]*)-?([0-9]*)
pub fn reg_modified_molecule(text: &str) -> Option<Vec<(usize, & str)>> {

    let text = text.as_bytes();

    let mut index = 0;

    while index < text.len() {

        //Start counter
        let mut counter = 0;

        //First capture started
        let first_cap_start = index + counter;

        //Character set, [A-Z]
        if index + counter > text.len() { //Bounds check. If this fails, there cannot possibly be a match at `index` so continue
            index += 1;
            continue;
        }

        if !(text[index+counter] >= 'A' as u8 && text[index+counter] <= 'Z' as u8) {
            index += 1;
            continue;
        }

        counter += 1;


        //Second character set and repeater [a-z]*
        if index + counter > text.len() {
            index += 1;
            continue;
        }
        for ch in &text[index+counter..] {
            if !(*ch >= 'a' as u8 && *ch <= 'z' as u8) {
                break;
            }
            counter += 1;
        }

        //First capture ended
        let first_cap_end = index + counter;

        //Literal character and repeater -?
        if text[index+counter] == '-' as u8 {
            counter += 1;
        }

        //Second capture started
        let second_cap_start = index + counter;

        //Third character set and repeater [0-9]*
        if index + counter > text.len() {
            index += 1;
            continue;
        }
        for ch in &text[index+counter..] {
            if !(*ch >= '0' as u8 && *ch <= '9' as u8) {
                break;
            }
            counter += 1;
        }

        //Second capture ended
        let second_cap_end = index + counter;

        unsafe {
            return Some(vec![(first_cap_start, std::str::from_utf8_unchecked(&text[first_cap_start..second_cap_end])),
                             (first_cap_start, std::str::from_utf8_unchecked(&text[first_cap_start..first_cap_end])),
                             (second_cap_start, std::str::from_utf8_unchecked(&text[second_cap_start..second_cap_end]))]);
        }

    }

    None

}

//er(([a-f][fd])(hell))[4-6]g+
pub fn reg_nested_captures(text: & str) -> Option<Vec<(usize, & str)>> {
    let text = text.as_bytes();

    let mut index = 0;

    while index < text.len() {


        //Start counter
        let mut counter = 0;

        //Multiple contiguous literal characters 'er'
        if index + counter + 2 > text.len() { //Bounds check. If this fails, there cannot possibly be a match at `index` so continue
            index += 1;
            continue;
        }

        if !(text[index+counter] == 'e' as u8 && text[index+counter+1] == 'r' as u8) {
            index += 1;
            continue;
        }

        counter += 2;

        //First capture started
        let first_cap_start = index + counter;

        //Second capture started
        let second_cap_start = index + counter;


        //Character set, [a-f]
        if index + counter > text.len() { //Bounds check. If this fails, there cannot possibly be a match at `index` so continue
            index += 1;
            continue;
        }

        if !(text[index+counter] >= 'a' as u8 && text[index+counter] <= 'f' as u8) {
            index += 1;
            continue;
        }

        counter += 1;

        //Character set, [fd]
        if index + counter > text.len() { //Bounds check. If this fails, there cannot possibly be a match at `index` so continue
            index += 1;
            continue;
        }

        if !(text[index+counter] == 'f' as u8 || text[index+counter] == 'd' as u8) {
            index += 1;
            continue;
        }

        counter += 1;

        //Second capture Ended
        let second_cap_end = index + counter;

        //Third capture started
        let third_cap_start = index + counter;

        //Multiple contiguous literal characters 'hell'
        if index + counter + 4 > text.len() { //Bounds check. If this fails, there cannot possibly be a match at `index` so continue
            index += 1;
            continue;
        }

        if !(text[index+counter] == 'h' as u8 && text[index+counter+1] == 'e' as u8 && text[index+counter+2] == 'l' as u8 && text[index+counter+3] == 'l' as u8) {
            index += 1;
            continue;
        }

        counter += 4;

        //Third capture started
        let third_cap_end = index + counter;

        //First capture started
        let first_cap_end = index + counter;

        //Character set, [4-6]
        if index + counter > text.len() { //Bounds check. If this fails, there cannot possibly be a match at `index` so continue
            index += 1;
            continue;
        }

        if !(text[index+counter] >= '4' as u8 && text[index+counter] <= '6' as u8) {
            index += 1;
            continue;
        }

        counter += 1;

        //Literal character with one or more repeater - 'g+'
        if index + counter > text.len() {
            index += 1;
            continue;
        }
        {
            let mut found = false;

            for ch in &text[index + counter..] {
                if !(*ch == 'g' as u8) {
                    break;
                }
                found = true;
                counter += 1;
            }

            if !found {
                index += 1;
                continue;
            }
        }

        unsafe {
            return Some(vec![(first_cap_start, std::str::from_utf8_unchecked(&text[index..index+counter])),
                             (first_cap_start, std::str::from_utf8_unchecked(&text[first_cap_start..first_cap_end])),
                             (second_cap_start, std::str::from_utf8_unchecked(&text[second_cap_start..second_cap_end])),
                             (second_cap_start, std::str::from_utf8_unchecked(&text[third_cap_start..third_cap_end]))]);
        }

    }
    None
}

pub fn reg_captures_with_repeats(text: & str) {

}