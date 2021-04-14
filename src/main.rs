use ansi_term::Colour::{Blue, Cyan, Green, Purple, Red, White, Yellow, RGB};
use ansi_term::{ANSIString, ANSIStrings};
use std::fmt;
use std::vec;

#[derive(Debug)]
enum Color {
    White,
    Orange,
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

fn color_first_color_letter(color: &Color) -> ANSIString {
    let first_letter = color.to_string().chars().next().unwrap().to_string();
    return match *color {
        Color::Blue => Blue.paint(first_letter),
        Color::Orange => RGB(255, 165, 0).paint(first_letter),
        Color::Cyan => Cyan.paint(first_letter),
        Color::Green => Green.paint(first_letter),
        Color::Purple => Purple.paint(first_letter),
        Color::Red => Red.paint(first_letter),
        Color::White => White.paint(first_letter),
        Color::Yellow => Yellow.paint(first_letter),
    };
}

fn fancy_print_guess(guess: &[Color]) {
    let mut vec_colors: Vec<ANSIString> = Vec::new();

    for color in guess {
        vec_colors.push(color_first_color_letter(color));
    }
    println!("{}", ANSIStrings(&vec_colors));
}

fn main() {
    let guess = vec![
        Color::Orange,
        Color::White,
        Color::Red,
        Color::Yellow,
        Color::Green,
        Color::Blue,
        Color::Cyan,
        Color::Purple,
    ];
    fancy_print_guess(&guess);
}
