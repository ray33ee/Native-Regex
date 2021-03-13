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
            return Some(vec![(index, std::str::from_utf8_unchecked(&text[index..index+counter])),
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
            return Some(vec![(index, std::str::from_utf8_unchecked(&text[index..index+counter])),
                             (first_cap_start, std::str::from_utf8_unchecked(&text[first_cap_start..first_cap_end])),
                             (second_cap_start, std::str::from_utf8_unchecked(&text[second_cap_start..second_cap_end])),
                             (second_cap_start, std::str::from_utf8_unchecked(&text[third_cap_start..third_cap_end]))]);
        }

    }
    None
}

//(hello)*t
pub fn reg_captures_with_repeats(text: & str) -> Option<Vec<(usize)>> {

    let text = text.as_bytes();

    let mut index = 0;

    'outer: while index < text.len() {

        //Start counter
        let mut counter = 0;

        //Capture group with repeater *
        if index + counter > text.len() {
            index += 1;
            continue;
        }
        let mut first_cap_start = None;
        let mut first_cap_end = None;

        'inner: for _ in &text[index+counter..] {

            first_cap_start = Some(index + counter);

            //Multiple contiguous literal characters 'hello'
            if index + counter + 5 > text.len() { //Bounds check. If this fails, there cannot possibly be a match at `index` so continue
                break;
            }

            if !(text[index+counter] == 'h' as u8 && text[index+counter+1] == 'e' as u8 && text[index+counter+2] == 'l' as u8 && text[index+counter+3] == 'l' as u8 && text[index+counter+4] == 'o' as u8) {
                break ;
            }

            counter += 5;

            first_cap_end = Some(index + counter);


        }

        //Individual Literal character
        if index + counter >= text.len() {
            index += 1;
            continue;
        }

        if !(text[index+counter] == 't' as u8) {
            index += 1;
            continue;
        }

        counter += 1;

        unsafe {
            return Some(vec![(index),
                             (first_cap_start.unwrap_or(0))]);
        }

    }

    None

}

//((tim)*ber)*m
pub fn reg_double_nested(text: &str) -> Option<[Option<(usize, & str)>; 3]> {
    let text = text.as_bytes();

    let mut index = 0;

    let mut captures: [Option<(usize, & str)>; 3] = [None; 3];

    while index < text.len() {

        //Start counter
        let mut counter = 0;

        let capture_zero_start = index;
        let mut capture_first_start = index;
        let mut capture_second_start = index;


        for _ in &text[index+counter..] {


            let mut capture_first_start = index + counter;

            for _ in &text[index+counter..] {


                let mut capture_second_start = index + counter;


                //Multiple contiguous literal characters 'tim'
                if index + counter + 3 > text.len() { //Bounds check. If this fails, there cannot possibly be a match at `index` so continue
                    break;
                }

                if !(text[index+counter] == 't' as u8 && text[index+counter+1] == 'i' as u8 && text[index+counter+2] == 'm' as u8) {
                    break ;
                }

                counter += 3;

                unsafe {
                    captures[2] = Some((capture_second_start, std::str::from_utf8_unchecked(&text[capture_second_start..index + counter])));
                }

            }

            //Multiple contiguous literal characters 'ber'
            if index + counter + 3 > text.len() { //Bounds check. If this fails, there cannot possibly be a match at `index` so continue
                break;
            }

            if !(text[index+counter] == 'b' as u8 && text[index+counter+1] == 'e' as u8 && text[index+counter+2] == 'r' as u8) {
                break;
            }

            counter += 3;

            unsafe {
                captures[1] = Some((capture_first_start, std::str::from_utf8_unchecked(&text[capture_first_start..index + counter])));
            }

        }

        //Multiple contiguous literal characters 'ber'
        if index + counter > text.len() { //Bounds check. If this fails, there cannot possibly be a match at `index` so continue
            index += 1;
            continue;
        }

        if !(text[index+counter] == 'm' as u8) {
            index += 1;
            continue;
        }

        counter += 1;

        unsafe {
            captures[0] = Some((capture_zero_start, std::str::from_utf8_unchecked(&text[capture_zero_start..index + counter])));
        }

        return Some(captures);
    }


    None
}

//(((tim)*ber)*m)*fin
pub fn reg_triple_nested(text: &str) -> Option<Vec<Option<(usize, & str)>>> {

    None
}

//[0-9]{5}[a-z]
pub fn reg_exactly_n(str_text: &str) -> Option<[Option<(usize, & str)>; 1]> {
    let text = str_text.as_bytes();

    let mut index = 0;

    let mut captures: [Option<(usize, & str)>; 1] = [None; 1];

    while index < text.len() {

        //Start counter
        let mut counter = 0;

        let capture_0_start = index + counter;

        {
            let mut match_count = 0;

            for ch in &text[index + counter..] {
                if index + counter + (1 - 1) > text.len() { //Bounds check. If this fails, there cannot possibly be a match at \`index\` so continue
                    break;
                }

                if !(*ch >= '0' as u8 && *ch <= '9' as u8) {
                    break
                }
                counter += 1;

                match_count += 1;

                if match_count == 5 {
                    break;
                }
            }

            if match_count < 5 {
                index += 1;
                continue;
            }


        }

        if index + counter + (1 - 1) > text.len() { //Bounds check. If this fails, there cannot possibly be a match at \`index\` so continue
            break;
        }

        if !(text[index+counter] >= 'a' as u8 && text[index+counter] <= 'z' as u8) {
            break
        }
        counter += 1;


        captures[0] = Some((index + counter, &str_text[capture_0_start..index+counter]));

        return Some(captures);
    }


    None
}

//(?:[0-9][A-Z]){3-6}[a-z]
pub fn reg_range_n_m(str_text: &str) -> Option<[Option<(usize, & str)>; 1]> {
    let text = str_text.as_bytes();

    let mut index = 0;

    let mut captures: [Option<(usize, & str)>; 1] = [None; 1];

    while index < text.len() {

        //Start counter
        let mut counter = 0;

        let capture_0_start = index + counter;

        {
            let mut match_count = 0;

            for _ in &text[index + counter..] {

                //let saved_count = counter;

                if index + counter + (1 - 1) > text.len() { //Bounds check. If this fails, there cannot possibly be a match at \`index\` so continue
                    //counter = saved_count;
                    break;
                }

                if !(text[index+counter] >= '0' as u8 && text[index+counter] <= '9' as u8) {
                    //counter = saved_count;
                    break;
                }
                counter += 1;

                if index + counter + (1 - 1) > text.len() { //Bounds check. If this fails, there cannot possibly be a match at \`index\` so continue
                    //counter = saved_count;
                    break;
                }

                if !(text[index+counter] >= 'A' as u8 && text[index+counter] <= 'Z' as u8) {
                    //counter = saved_count;
                    break;
                }
                counter += 1;

                match_count += 1;

                if match_count == 6 {
                    break;
                }
            }

            if match_count < 3 {
                index += 1;
                continue;
            }

            println!("Matched 3-6");

        }

        if index + counter + (1 - 1) > text.len() { //Bounds check. If this fails, there cannot possibly be a match at \`index\` so continue
            index += 1;
            continue;
        }

        if !(text[index+counter] >= 'a' as u8 && text[index+counter] <= 'z' as u8) {
            index += 1;
            continue;
        }
        counter += 1;


        captures[0] = Some((index + counter, &str_text[capture_0_start..index+counter]));

        return Some(captures);
    }


    None
}

//[0-9]{7,}[a-z]
pub fn reg_n_above(str_text: &str) -> Option<[Option<(usize, & str)>; 1]> {
    let text = str_text.as_bytes();

    let mut index = 0;

    let mut captures: [Option<(usize, & str)>; 1] = [None; 1];

    while index < text.len() {

        //Start counter
        let mut counter = 0;

        let capture_0_start = index + counter;

        {
            let mut match_count = 0;

            for ch in &text[index + counter..] {
                if index + counter + (1 - 1) > text.len() { //Bounds check. If this fails, there cannot possibly be a match at \`index\` so continue
                    break;
                }

                if !(*ch >= '0' as u8 && *ch <= '9' as u8) {
                    break
                }
                counter += 1;

                match_count += 1;
            }

            if match_count < 7 {
                index += 1;
                continue;
            }


        }

        if index + counter + (1 - 1) > text.len() { //Bounds check. If this fails, there cannot possibly be a match at \`index\` so continue
            break;
        }

        if !(text[index+counter] >= 'a' as u8 && text[index+counter] <= 'z' as u8) {
            break
        }
        counter += 1;

        captures[0] = Some((index + counter, &str_text[capture_0_start..index+counter]));

        return Some(captures);
    }


    None
}