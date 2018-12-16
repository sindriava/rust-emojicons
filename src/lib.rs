extern crate phf;
extern crate regex;
#[macro_use] 
extern crate lazy_static;

use std::fmt;
use regex::{
    Regex,
    Captures,
};

include!(concat!(env!("OUT_DIR"), "/emojis.rs"));

/// Macro for compile-time emoji lookup
///
/// This macro will expand to the string stored in `EMOJIS` on compile-time.
/// This doesn't introduce any overhead, but is useful to prevent pasting of
/// unicode into the code.
///
/// # Example
///
/// ```rust
/// #[macro_use] extern crate emojicons;
/// 
/// # fn main() {
/// assert_eq!(emoji!("cat").to_string(), "\u{01F431}");
/// # }
/// ```
#[macro_export]
macro_rules! emoji {
    ($e: expr) => (
        $crate::EMOJIS.get(&format!(":{}:", $e)[..]).unwrap_or(&$e);
    )
}


// replaces all emojis with their unicode representation
pub fn replace_all_emojis(input: &str) -> String{

    // compiles expression only once
    // and will resuse reference every time
    lazy_static! {
        static ref re: Regex = Regex::new(":([a-zA-Z0-9_+-]+):").unwrap();
    }
    
    // replaces all references to emojis
    let result = re.replace_all(input, |capts: &Captures| {
        let sym = capts.get(0).unwrap().as_str();

        // returns that string to bind to result
        // as either the unicode emoji or the string
        match EMOJIS.get(sym) {
            Some(e) => format!("{}", e),
            None    => sym.to_string()
        }
    });

    // transfers ownership of string
    result.into_owned()
}




/// Newtype used for substituting emoji codes for emoji
///
/// Leaves the notation intact if a corresponding emoji is not found in the
/// lookup table.
pub struct EmojiFormatter<'a>(pub &'a str);

impl<'a> std::fmt::Display for EmojiFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // compiles expression only once
        // and will resuse reference every time
        lazy_static! {
            static ref re: Regex = Regex::new(":([a-zA-Z0-9_+-]+):").unwrap();
        }
        
        // replaces all references to emojis
        let result = re.replace_all(self.0, |capts: &Captures| {
            let sym = capts.get(0).unwrap().as_str();

            // returns that string to bind to result
            // as either the unicode emoji or the string
            match EMOJIS.get(sym) {
                Some(e) => format!("{}", e),
                None    => sym.to_string()
            }
        });

        write!(f, "{}", result)
    }
}