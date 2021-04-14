use ansi_term::Colour;
use std::io::{self, BufRead, Write};

#[derive(Debug)]
enum Color {
    Green,
    Red,
    Blue,
    Yellow,
    Purple,
    Cyan,
    Orange,
    White,
}

fn main() {
    let solution: Vec<Color> = vec![
        Color::Green,
        Color::Red,
        Color::Blue,
        Color::Purple,
        Color::Cyan,
    ];

    for line in io::stdin().lock().lines() {
        let guess = parse_colors(line.unwrap());
        fancy_print_guess(&guess);
        io::stdout().flush().unwrap();
    }
}

fn fancy_print_guess(guess: &[Color]) {
    for c in guess.iter() {
        match c {
            Color::Green => print!("{}", Colour::Green.paint("G")),
            Color::Red => print!("{}", Colour::Red.paint("R")),
            Color::Blue => print!("{}", Colour::Blue.paint("B")),
            Color::Yellow => print!("{}", Colour::Yellow.paint("Y")),
            Color::Purple => print!("{}", Colour::Purple.paint("P")),
            Color::Cyan => print!("{}", Colour::Cyan.paint("C")),
            Color::Orange => print!("{}", Colour::RGB(255, 165, 0).paint("O")),
            Color::White => print!("{}", Colour::White.paint("W")),
        }
    }
    print!("{}", '\n');
}

fn parse_colors(raw_colors: String) -> Vec<Color> {
    let char_vec: Vec<char> = raw_colors.chars().collect();
    let mut guess: Vec<Color> = vec![];

    for c in char_vec.iter() {
        match c {
            'R' => guess.push(Color::Red),
            'G' => guess.push(Color::Green),
            'B' => guess.push(Color::Blue),
            '\n' => (),
            _ => panic!(),
        };
    }

    return guess;
}
