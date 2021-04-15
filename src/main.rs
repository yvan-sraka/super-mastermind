use ansi_term::Colour::{Blue, Cyan, Green, Purple, Red, White, Yellow};
use std::str::FromStr;
use rand::prelude::*;

#[derive(Debug, PartialEq)]
enum Color {
    R,
    G,
    B,
    Y,
    C,
    P,
    O,
    V,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Color, Self::Err> {
        match s {
            "R" => Ok(Color::R),
            "B" => Ok(Color::B),
            "G" => Ok(Color::G),
            "O" => Ok(Color::O),
            "P" => Ok(Color::P),
            "V" => Ok(Color::V),
            "Y" => Ok(Color::Y),
            "C" => Ok(Color::C),
            _ => Err(()),
        }
    }
}

fn create_ainsi_color(char_color: &str, color: &Color) -> String {
    match color {
        Color::R => Red.paint(char_color).to_string(),
        Color::B => Blue.paint(char_color).to_string(),
        Color::G => Green.paint(char_color).to_string(),
        Color::O => White.paint(char_color).to_string(),
        Color::P => Purple.paint(char_color).to_string(),
        Color::V => White.paint(char_color).to_string(),
        Color::Y => Yellow.paint(char_color).to_string(),
        Color::C => Cyan.paint(char_color).to_string(),
    }
}

fn fancy_print_guess(guess: &[Color]) {
    let mut combination = String::from("");

    for elem in guess {
        match elem {
            Color::R => combination.push_str(&create_ainsi_color("R", &Color::R)),
            Color::B => combination.push_str(&create_ainsi_color("B", &Color::B)),
            Color::G => combination.push_str(&create_ainsi_color("G", &Color::G)),
            Color::O => combination.push_str(&create_ainsi_color("O", &Color::O)),
            Color::P => combination.push_str(&create_ainsi_color("P", &Color::P)),
            Color::V => combination.push_str(&create_ainsi_color("V", &Color::V)),
            Color::Y => combination.push_str(&create_ainsi_color("Y", &Color::Y)),
            Color::C => combination.push_str(&create_ainsi_color("C", &Color::C)),
        }
    }

    println!("{}", combination)
}

fn read_input() -> String {
    let mut combination = String::new();
    std::io::stdin().read_line(&mut combination).unwrap();

    combination
}

fn parse_combination(user_combination: String) -> Vec<Color> {
    let mut combination_arr: Vec<Color> = Vec::new();

    // Parse combination
    let mut arr: Vec<&str> = user_combination.trim().split("").collect();
    // Remove empty string
    arr = arr.into_iter().filter(|&s| s != "").collect();
    for s in arr {
        combination_arr.push(Color::from_str(s.trim()).unwrap());
    }
    return combination_arr;
    
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut count: i32 = 0;
    for i in 0..secret.len() {
        if secret[i] == guess[i]{
            count += 1
        }
    }
    count
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut count: i32 = 0;
    for i in 0..secret.len() {
        if secret[i] != guess[i]{
            count += 1
        }
    }
    count
}

fn random_secret() -> Vec<Color> {
    let mut range = rand::thread_rng();
    let mut colors: Vec<Color> = Vec::new();
    for _ in 0..5 {
        let random_number = range.gen_range(0..7);
        match random_number {
            0 => colors.push(Color::R),
            1 => colors.push(Color::G),
            2 => colors.push(Color::B),
            3 => colors.push(Color::P),
            4 => colors.push(Color::Y),
            5 => colors.push(Color::P),
            6 => colors.push(Color::V),
            7 => colors.push(Color::O),
            _ => {}
        }
    }

    return colors;
}

fn main() {
    let guess = random_secret(); // Create a combination of color
    let mut guess_number = 1;
    let mut running = true;

    while running == true {
    
        println!("Essai {}", guess_number);
        println!("Type your color combination : ");
        let user_input = read_input();

        if user_input.clone().trim().len() != 5 {
            println!("Please type 5 characters");
            continue;
        } 

        guess_number += 1;

        // Print user combination with colors
        fancy_print_guess(&parse_combination(user_input.clone()));

        if parse_combination(user_input.clone()) == guess {
            println!("Congratulations !!!! You won after {} guess.", guess_number);
            running = false;
            continue;
        }

        println!("{} color(s) are/is correctly placed", number_of_well_placed_pawns(&guess, &parse_combination(user_input.clone())));
        println!("{} color(s) are/is not correctly placed", number_of_not_well_placed_pawns(&guess, &parse_combination(user_input)));
    }
}
