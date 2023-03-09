use efcl::{color, Color, bold};

fn main() {
    println!("{}!", bold!(&color!(Color::RED, "EFCL").to_string()));
    println!("Hello, {}!", color!(Color::BLUE, "world"));
}
