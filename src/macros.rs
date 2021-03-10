#![macro_use]

#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}

#[macro_export]
macro_rules! indexed_native_searcher {
    ( $name:ident, $first:expr, $( $characters:expr ; $index:expr ),* ) => {
        fn $name(text: & str) -> Option<(usize, usize)> {

            let mut index = 0;

            const SUBSTR_LEN: usize = <[()]>::len(&[$(replace_expr!(($characters) ())),*]);

            let text = text.as_bytes();

            while index + SUBSTR_LEN < text.len() {

                if text[index] == $first as u8 $(&& text[index+$index] == $characters as u8)* {
                    return Some((index, index + SUBSTR_LEN + 1))
                }

                index += 1;

            }

            None
        }
    };
}

#[macro_export]
macro_rules! native_searcher {
    ( $name:ident, $( $characters:expr ),* ) => {
        fn $name(text: & str) -> Option<(usize, usize)> {
            #![allow(unused_assignments)]

            let mut index = 0;
            let mut counter = 0;

            const SUBSTR_LEN: usize = <[()]>::len(&[$(replace_expr!(($characters) ())),*]);

            let text = text.as_bytes();

            while index + SUBSTR_LEN - 1 < text.len() {

                $(
                if text[index + counter] != $characters as u8 {
                    index += 1;
                    counter = 0;
                    continue;
                }
                else {
                    counter += 1;
                }
                )*

                return Some((index, index + SUBSTR_LEN));
            }
            None
        }
    };
}
