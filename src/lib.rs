//! EFCL - Efficiency First Color Library
//!
//! The most simple, small, and fast terminal color text library.
//!
//! ```
//! use efcl::{color, Color, bold};
//!
//! fn main() {
//!     println!("Hello, {}!", color!(Color::BLUE, "world"));
//!
//!     println!("{}!", bold!(&color!(Color::RED, "EFCL").to_string()));
//! }
//! ```

/// Color is an enum that represents the ANSI escape code for terminal colors
#[derive(Copy, Clone)]
pub enum Color {
    BLACK = 30,
    RED = 31,
    GREEN = 32,
    YELLOW = 33,
    BLUE = 34,
    PURPLE = 35,
    CYAN = 36,
    LIGHTGRAY = 37,
}

impl Color {
    pub fn as_str(&self) -> &'static str {
        match self {
            Color::BLACK => "30",
            Color::RED => "31",
            Color::GREEN => "32",
            Color::YELLOW => "33",
            Color::BLUE => "34",
            Color::PURPLE => "35",
            Color::CYAN => "36",
            Color::LIGHTGRAY => "37",
        }
    }
}

/// Return a String with a Color added to it
///
/// # Examples
///
/// ```
/// let red_text: String = color!(Color::RED, "This is RED!!");
/// ```
#[macro_export]
macro_rules! color {
    ($a:expr, $b:expr) => {{
        "\x1B[".to_owned() + $a.as_str() + "m" + $b + "\x1B[0m"
    }};
}

/// Return a String that is bold
///
/// # Examples
///
/// ```
/// let bold_text: String = bold!("This is BOLD!!");
/// ```
#[macro_export]
macro_rules! bold {
    ($a:expr) => {{
        "\x1B[1m".to_owned() + $a + "\x1B[0m"
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_macro_test() {
        let c: String = color!(Color::RED, "This is RED!!");
        assert_eq!(c, "\x1B[31mThis is RED!!\x1B[0m");
    }

    #[test]
    fn bold_macro_test() {
        let bold_text: String = bold!("This is BOLD!!");
        assert_eq!(bold_text, "\x1B[1mThis is BOLD!!\x1B[0m");

        assert_eq!(
            bold!(&color!(Color::RED, "Hello")),
            "\u{1b}[1m\u{1b}[31mHello\u{1b}[0m\u{1b}[0m"
        );
    }
}
