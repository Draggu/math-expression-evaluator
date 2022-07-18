use super::tokens;
use crate::common::token::Token;
use std::iter;

macro_rules! apply_matchers {
    ($input:expr,$($matcher:path),*) => {
        {
            let mut token_match = None;

            $(
                token_match = match token_match {
                    None => $matcher($input),
                    r => r,
                };
            )*

            token_match
        }
    };
}

pub fn tokenize(input_str: &str) -> Result<Vec<Token>, String> {
    iter::from_fn({
        let mut index = 0;

        move || match apply_matchers![
            &input_str[index..],
            tokens::operator,
            tokens::white_space,
            tokens::identificator,
            tokens::literal,
            tokens::bracket,
            tokens::comma
        ] {
            None => {
                if input_str.len() <= index {
                    None
                } else {
                    Some(Some(Err(format!(
                        "{}{}",
                        "unrecognised char at index: ",
                        index + 1
                    ))))
                }
            }
            Some(result) => {
                index += result.offset;

                match result.matched {
                    None => Some(None),
                    Some(r) => Some(Some(Ok(r))),
                }
            }
        }
    })
    .filter_map(|r| r)
    .collect()
}
