use ansi_term::Colour;
use cli_table::{format::Justify, print_stdout, Table, WithTitle};
use std::{
    collections::HashMap,
    io::{self, BufRead, Write},
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

#[derive(Debug, Clone, Table)]
struct MastermindGuessResult {
    #[table(title = "Attempts", justify = "Justify::Right")]
    pub tries_count: i32,
    #[table(title = "Guess")]
    pub guess: String,
    #[table(title = "Result")]
    pub result: String,
    #[table(title = "Well placed pawns")]
    pub well_placed_pawns: i32,
    #[table(title = "Not well placed pawns")]
    pub not_well_placed_pawns: i32,
}

fn main() {
    let secret: Vec<Color> = vec![Color::Green, Color::Red, Color::Blue, Color::Purple];
    let mut guess_count = 0;
    let mut guess_results: Vec<MastermindGuessResult> = vec![];

    println!("Welcome to the mastermind!");
    println!("You must find the secret combination of 4 colors, composed of any of these: GRBYPCW");
    println!("Good luck...");
    print!("Enter guess ➜ ");

    io::stdout().flush().unwrap();

    for line in io::stdin().lock().lines() {
        let parsed_combination_guess = parse_colors(line.unwrap());
        match parsed_combination_guess {
            Ok(combination) => {
                guess_count += 1;
                let correct_guess = guess_match_solution(&combination, &secret);
                let result = MastermindGuessResult {
                    guess: fancy_print_guess(&combination),
                    result: match correct_guess {
                        true => "CORRECT! ✅ ".to_string(),
                        false => "WRONG! ❌ ".to_string(),
                    },
                    well_placed_pawns: number_of_well_placed_pawns(&secret, &combination),
                    not_well_placed_pawns: number_of_not_well_placed_pawns(&secret, &combination),
                    tries_count: guess_count,
                };

                guess_results.push(result.clone());

                print_stdout(guess_results.with_title()).unwrap();

                if correct_guess {
                    println!("You beat the mastermind!");
                    break;
                }
                if result.tries_count >= 10 {
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
