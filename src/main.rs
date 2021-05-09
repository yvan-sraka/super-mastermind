use ansi_term::Colour;
use cli_table::{format::Justify, print_stdout, Table, WithTitle};
use std::{
    collections::HashMap,
    io::{self, BufRead, Write},
};
extern crate rand;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0..=7) {
            0 => Color::Green,
            1 => Color::Red,
            2 => Color::Blue,
            3 => Color::Yellow,
            4 => Color::Purple,
            5 => Color::Cyan,
            6 => Color::Orange,
            _ => Color::White,
        }
    }
}

#[derive(Debug, Clone, Table)]
struct MastermindGuessResult {
    #[table(title = "Attempts", justify = "Justify::Right")]
    pub attempts: i32,
    #[table(title = "Guess")]
    pub guess: String,
    #[table(title = "Result")]
    pub result: String,
    pub valid: bool,
    #[table(title = "Well placed pawns")]
    pub well_placed_pawns: i32,
    #[table(title = "Not well placed pawns")]
    pub not_well_placed_pawns: i32,
}

fn main() {
    let secret = get_rand_secret();
    let mut attempts = 0;
    let mut guess_results: Vec<MastermindGuessResult> = vec![];

    println!("Welcome to the mastermind!");
    println!("You must find the secret combination of 4 colors, composed of any of these: GRBYPCW");
    println!("Good luck...");
    print!("Enter guess ➜ ");

    io::stdout().flush().unwrap();

    // Handle input until victory or max attempts
    for line in io::stdin().lock().lines() {
        let parsed_combination_guess = parse_colors(line.unwrap());
        match parsed_combination_guess {
            Ok(combination) => {
                attempts += 1;

                let result = handle_guess(&combination, &secret, attempts);

                // Print table
                guess_results.push(result.clone());
                print_stdout(guess_results.with_title()).unwrap();

                if result.valid {
                    println!("You beat the mastermind!");
                    break;
                }
                if result.attempts >= 10 {
                    println!("You failed to beat the mastermind in 10 attempts. Try again!");
                    break;
                }
            }
            Err(err) => eprintln!("{}", err),
        }

        print!("Enter guess ➜ ");
        io::stdout().flush().unwrap();
    }
}

fn get_rand_secret() -> Vec<Color> {
    return vec![
        rand::random(),
        rand::random(),
        rand::random(),
        rand::random(),
    ];
}

fn handle_guess(combination: &[Color], secret: &[Color], attempts: i32) -> MastermindGuessResult {
    let valid = guess_match_solution(&combination, &secret);
    return MastermindGuessResult {
        guess: fancy_print_guess(&combination),
        valid,
        result: match valid {
            true => "CORRECT! ✅ ".to_string(),
            false => "WRONG! ❌ ".to_string(),
        },
        well_placed_pawns: number_of_well_placed_pawns(&secret, &combination),
        not_well_placed_pawns: number_of_not_well_placed_pawns(&secret, &combination),
        attempts: attempts,
    };
}

fn guess_match_solution(secret: &[Color], combination: &[Color]) -> bool {
    return number_of_well_placed_pawns(secret, combination) == secret.len() as i32;
}

fn number_of_well_placed_pawns(secret: &[Color], combination: &[Color]) -> i32 {
    return combination
        .iter()
        .zip(secret)
        .filter(|&(c, s)| c == s)
        .count() as i32;
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    return guess.iter().filter(|c| secret.contains(c)).count() as i32;
}

fn fancy_print_guess(guess: &[Color]) -> String {
    let mut fancy_guess = "".to_owned();

    for c in guess.iter() {
        match c {
            Color::Green => fancy_guess.push_str(&Colour::Green.paint("G")),
            Color::Red => fancy_guess.push_str(&Colour::Red.paint("R")),
            Color::Blue => fancy_guess.push_str(&Colour::Blue.paint("B")),
            Color::Yellow => fancy_guess.push_str(&Colour::Yellow.paint("Y")),
            Color::Purple => fancy_guess.push_str(&Colour::Purple.paint("P")),
            Color::Cyan => fancy_guess.push_str(&Colour::Cyan.paint("C")),
            Color::Orange => fancy_guess.push_str(&Colour::RGB(255, 165, 0).paint("O")),
            Color::White => fancy_guess.push_str(&Colour::White.paint("W")),
        }
    }

    // Color are broken :'(
    return fancy_guess;
}

fn parse_colors(raw_colors: String) -> Result<Vec<Color>, String> {
    let colors: Vec<char> = raw_colors.chars().collect();

    if colors.len() != 4 {
        return Err(format!("You must guess 4 colors, not {}!", colors.len()).to_string());
    }

    let mut colors_map: HashMap<char, Color> = HashMap::new();
    colors_map.insert('R', Color::Red);
    colors_map.insert('G', Color::Green);
    colors_map.insert('B', Color::Blue);
    colors_map.insert('Y', Color::Yellow);
    colors_map.insert('P', Color::Purple);
    colors_map.insert('C', Color::Cyan);
    colors_map.insert('O', Color::Orange);
    colors_map.insert('W', Color::White);

    let mut guess: Vec<Color> = vec![];

    for color in colors.iter() {
        if !colors_map.contains_key(color) {
            return Err(format!("Invalid color: {}", color).to_string());
        }
        guess.push(colors_map[color]);
    }

    return Ok(guess);
}
