use std::fmt;
use std::vec;

#[derive(Debug)]
enum Color {
    White,
    Black,
    Blue,
    Green,
    Red,
    Yellow,
    Purple,
    Cyan,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn fancy_print_guess(guess: &[Color]) {
    let mut vec_colors: Vec<char> = Vec::new();

    for color in guess {
        vec_colors.push(color.to_string().chars().next().unwrap());
    }
    let color_str: String = vec_colors.into_iter().collect();
    println!("{:?}", color_str);
}

fn main() {
    let guess = vec![
        Color::Black,
        Color::White,
        Color::Black,
        Color::Green,
        Color::Blue,
    ];
    fancy_print_guess(&[
        Color::Red,
        Color::Red,
        Color::Blue,
        Color::Yellow,
        Color::Green,
    ]);
}
