use ansi_term::Colour::Red;
use ansi_term::Colour::Blue;
use ansi_term::Colour::Green;
use ansi_term::Colour::Yellow;
use ansi_term::Colour::Purple;
use ansi_term::Colour::White;
use ansi_term::Colour::Cyan;
use ansi_term::Colour::Black;
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

fn color_exist(haystack: char) -> bool {
    let colors: Vec<Colors> = vec![Colors::RED, Colors::BLUE, Colors::GREEN, Colors::YELLOW, Colors::PURPLE, Colors::WHITE, Colors::BLACK, Colors::CYAN];
    for color in colors {
        if color.value().contains(haystack) {
            return true;
        }
    }
    return false;
}

fn get_color(haystack: char) -> Colors {
    let colors: Vec<Colors> = vec![Colors::RED, Colors::BLUE, Colors::GREEN, Colors::YELLOW, Colors::PURPLE, Colors::WHITE, Colors::BLACK, Colors::CYAN];
    for color in colors {
        if color.value().contains(haystack) {
            return color;
        }
    }
    return Colors::GREEN;
}

fn is_won(colors: Vec<Colors>, hidden_combination: Vec<Colors>) -> bool {
    if colors.len() == 0 {
        return false;
    }
    for i in 0..hidden_combination.len() {
        if colors[i].value() != hidden_combination[i].value() {
            return false;
        }
    }
    return true;
}

fn number_of_well_placed_pawns(secret: Vec<Colors>, guess: Vec<Colors>) -> i32 {
    let mut placed = 0;
    if guess.is_empty() {
        return 0;
    }
    for i in 0..secret.len() {
        if guess[i].value() == secret[i].value() {
            placed += 1;
        }
    }
    return placed;
}

fn fancy_print_guess(vector_colors: Vec<Colors>) {
    for color in vector_colors {
        print!("{}", color.value());
    }
    println!();
}

fn number_of_not_well_placed_pawns(secret: Vec<Colors>, guess: Vec<Colors>) -> i32 {
    if guess.is_empty() {
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
    let base_colors: Vec<Colors> = vec![Colors::RED, Colors::BLUE, Colors::GREEN, Colors::CYAN, Colors::BLACK, Colors::WHITE, Colors::PURPLE, Colors::YELLOW];
    let mut guess: Vec<Colors> = vec![];
    for i in 0..8 {
        let mut rng = rand::thread_rng();
        guess.push(base_colors[rng.gen_range(0,8)]);
    }
    fancy_print_guess(guess.to_vec());
    let mut input_color = Vec::new();
    let mut turn = 0;
    let mut error = 0;
    while true {
        error = 0;
        let mut line = String::new();
        println!("Try to guess ! Turn number {} :", turn);
        let _b1 = std::io::stdin().read_line(&mut line).unwrap();
        line = line.replace("\n", "");
        if line.is_empty() {
            println!("Please type something");
        }
        for (i, c) in line.chars().enumerate() {
            if color_exist(c) {
                input_color.push(get_color(c));
            } else {
                println!("Error, the color with key {} does'nt exist at index {}", c, i);
                error = 1;
            }
        }
        if error == 0 {
            turn += 1;
        }
        if is_won(input_color.to_vec(), guess.to_vec()) {
            println!("You WON ! ");
            break;
        }
        let well_placed = number_of_well_placed_pawns(guess.to_vec(), input_color.to_vec());
        let not_well_placed = number_of_not_well_placed_pawns(guess.to_vec(), input_color.to_vec());
        fancy_print_guess(input_color.to_vec());
        println!("Number of well placed pawn, {}", well_placed);
        println!("Number of not well placed paws {}", not_well_placed);
    }
}
