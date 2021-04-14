use ansi_term::Colour::Red;
use ansi_term::Colour::Blue;
use ansi_term::Colour::Green;
use ansi_term::Colour::Yellow;
use ansi_term::Colour::Purple;
use ansi_term::Colour::White;
use ansi_term::Colour::Cyan;
use ansi_term::Colour::Black;
use std::io;
use std::io::BufRead;
use std::ptr::null;
use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Colors {
    BLUE,
    RED,
    GREEN,
    YELLOW,
    PURPLE,
    WHITE,
    CYAN,
    BLACK,
}

impl Colors {
    fn value(&self) -> String {
        match *self {
            Colors::RED => Red.paint("R").to_string(),
            Colors::BLUE => Blue.paint("B").to_string(),
            Colors::GREEN => Green.paint("G").to_string(),
            Colors::CYAN => Cyan.paint("C").to_string(),
            Colors::PURPLE => Purple.paint("P").to_string(),
            Colors::BLACK => Black.paint("D").to_string(),
            Colors::WHITE => White.paint("W").to_string(),
            Colors::YELLOW => Yellow.paint("Y").to_string()
        }
    }
}

fn colorExist(haystack: char) -> bool {
    let colors: Vec<Colors> = vec![Colors::RED, Colors::BLUE, Colors::GREEN, Colors::YELLOW, Colors::PURPLE, Colors::WHITE, Colors::BLACK, Colors::CYAN];
    for color in colors {
        if color.value().contains(haystack) {
            return true;
        }
    }
    return false;
}

fn getColor(haystack: char) -> Colors {
    let colors: Vec<Colors> = vec![Colors::RED, Colors::BLUE, Colors::GREEN, Colors::YELLOW, Colors::PURPLE, Colors::WHITE, Colors::BLACK, Colors::CYAN];
    for color in colors {
        if color.value().contains(haystack) {
            return color;
        }
    }
    return Colors::GREEN;
}

fn is_won(colors: Vec<Colors>, hiddenCombination: Vec<Colors>) -> bool {
    if colors.len() == 0 {
        return false;
    }
    for i in 0..hiddenCombination.len() {
        if colors[i].value() != hiddenCombination[i].value() {
            return false;
        }
    }
    return true;
}

fn number_of_well_placed_pawns(secret: Vec<Colors>, guess: Vec<Colors>) -> i32 {
    let mut placed = 0;
    if guess.len() == 0 {
        return 0;
    }
    for i in 0..secret.len() {
        if guess[i].value() == secret[i].value() {
            placed += 1;
        }
    }
    return placed;
}

fn fancy_print_guess(vectorColors: Vec<Colors>) {
    for color in vectorColors {
        print!("{}", color.value());
    }
    println!();
}

fn number_of_not_well_placed_pawns(secret: Vec<Colors>, guess: Vec<Colors>) -> i32 {
    if guess.len() == 0 {
        return secret.len() as i32;
    }
    let mut placed = 0;
    for i in 0..secret.len() {
        if guess[i].value() != secret[i].value() {
            placed += 1;
        }
    }
    return placed;
}

fn main() {
    let baseColors: Vec<Colors> = vec![Colors::RED, Colors::BLUE, Colors::GREEN, Colors::CYAN, Colors::BLACK, Colors::WHITE, Colors::PURPLE, Colors::YELLOW];
    let mut guess: Vec<Colors> = vec![];
    for i in 0..8 {
        let mut rng = rand::thread_rng();
        guess.push(baseColors[rng.gen_range(0,8)]);
    }
    println!("{:?}", guess.to_vec());
    fancy_print_guess(guess.to_vec());
    let mut inputColor = Vec::new();
    let mut turn = 0;
    let mut error = 0;
    while true {
        error = 0;
        let mut line = String::new();
        println!("Try to guess ! Turn number {} :", turn);
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        line = line.replace("\n", "");
        if line.len()== 0 {
            println!("Please type something");
        }
        for (i, c) in line.chars().enumerate() {
            if colorExist(c) {
                inputColor.push(getColor(c));
            } else {
                println!("Error, the color with key {} does'nt exist at index {}", c, i);
                error = 1;
            }
        }
        if error == 0 {
            turn += 1;
        }
        if (is_won(inputColor.to_vec(), guess.to_vec())) {
            println!("You WON ! ");
            break;
        }
        let well_placed = number_of_well_placed_pawns(guess.to_vec(), inputColor.to_vec());
        let not_well_placed = number_of_not_well_placed_pawns(guess.to_vec(), inputColor.to_vec());
        fancy_print_guess(inputColor.to_vec());
        println!("Number of well placed pawn, {}", well_placed);
        println!("Number of not well placed paws {}", not_well_placed);
    }
}
