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

#[macro_export]
macro_rules! color {
    ($a:expr, $b:expr) => {{
        format!("\x1B[{}m{}\x1B[0m", $a as u8, $b)
    }};
}

#[macro_export]
macro_rules! bold {
    ($a:expr) => {{
        format!("\x1B[1m{}\x1B[0m", $b)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_macro_test() {
        assert_eq!(
            color!(Color::RED, "This is RED!!"),
            "\x1B[31mThis is RED!!\x1B[0m"
        );
    }
}
