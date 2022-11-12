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
        let number = $a as u8;
        let num_str = number.to_string();
        "\x1B[".to_owned() + &num_str.to_owned() + "m" + $b + "\x1B[0m"
    }};
}

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
        assert_eq!(
            color!(Color::RED, "This is RED!!"),
            "\x1B[31mThis is RED!!\x1B[0m"
        );
    }

    #[test]
    fn bold_macro_test() {
        assert_eq!(bold!("This is BOLD!!"), "\x1B[1mThis is BOLD!!\x1B[0m");
    }
}
