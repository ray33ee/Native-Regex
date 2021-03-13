// Hard coded function to match regex '([A-Z][a-z]*)([0-9]*)'
pub fn testing(str_text: &str) -> Option<Vec<Option<(usize, & str)>>> {
    let text = str_text.as_bytes();

    let mut index = 0;

    let mut captures = vec![None; 3];

    while index < text.len() {

        //Start counter
        let mut counter = 0;

        let capture_0_start = index + counter;

        {

            let capture_1_start = index+counter;

            if index + counter + (1 - 1) > text.len() { index += 1; continue; }

            if !(text[index+counter] >= 'A' as u8 && text[index+counter] <= 'Z' as u8) { index += 1; continue; }

            counter += 1;

            for _ in &text[index + counter..] {
                if index + counter + (1 - 1) > text.len() { break; }

                if !(text[index+counter] >= 'a' as u8 && text[index+counter] <= 'z' as u8) { break; }

                counter += 1;


            }

            captures[1] = Some((index + counter, &str_text[capture_1_start..index + counter]));

        }

        {

            let capture_2_start = index+counter;

            for _ in &text[index + counter..] {
                if index + counter + (1 - 1) > text.len() { break; }

                if !(text[index+counter] >= '0' as u8 && text[index+counter] <= '9' as u8) { break; }

                counter += 1;


            }

            captures[2] = Some((index + counter, &str_text[capture_2_start..index + counter]));

        }



        captures[0] = Some((index + counter, &str_text[capture_0_start..index+counter]));

        return Some(captures);
    }


    None
}