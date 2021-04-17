use ansi_term::Colour;
use std::io::{self, BufRead};
use rand::prelude::*;


#[derive(Debug)]
#[allow(dead_code)]
#[derive(PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
    Magenta,
    Cyan,
}

// Prints Color by letter
fn fancy_print_guess(guess: &[Color]) {
    for color in guess {
        match color {
            Color::Red => print!("{}", Colour::Red.paint("R")),
            Color::Green => print!("{}", Colour::Green.paint("G")),
            Color::Blue => print!("{}", Colour::Blue.paint("B")),
            Color::Yellow => print!("{}", Colour::Yellow.paint("Y")),
            Color::Orange => print!("{}", Colour::RGB(255, 165, 0).paint("O")),
            Color::Purple => print!("{}", Colour::Purple.paint("P")),
            Color::Magenta => print!("{}", Colour::RGB(255, 0, 255).paint("M")),
            Color::Cyan => print!("{}", Colour::Cyan.paint("C")),
        }
    }
    println!();
}

// Return the solution the user guessed
fn parse_user_input(user_input: String) -> Vec<Color> {
    if user_input.len() != 5 {
        panic!("user_input is invalid ! Must be 5 letters");
    }
    let mut user_input_vec: Vec<Color> = Vec::new();
    for char in user_input.chars() {
        match char {
            'R' => user_input_vec.push(Color::Red),
            'G' => user_input_vec.push(Color::Green),
            'B' => user_input_vec.push(Color::Blue),
            'Y' => user_input_vec.push(Color::Yellow),
            'O' => user_input_vec.push(Color::Orange),
            'P' => user_input_vec.push(Color::Purple),
            'M' => user_input_vec.push(Color::Magenta),
            'C' => user_input_vec.push(Color::Cyan),
            '\n' => (),
            _ => panic!(),
        };
    }
    return user_input_vec;
}

// Returns the number of well placed colors guessed by the user
fn number_of_well_placed_pawns(secret: &[Color], user_guess: &[Color]) -> i32 {
    return secret.iter().zip(user_guess.iter()).filter(|&(guess, user_guess)| guess == user_guess).count() as i32;
}

fn number_of_not_well_placed_pawns(secret: &[Color], user_guess: &[Color]) -> i32 {
    return user_guess.iter().filter(|color| secret.contains(color)).count() as i32;
}
 
fn main() {
    
    // List of possible colors to guess
    let colors = vec![
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::Yellow,
        Color::Orange,
        Color::Purple,
        Color::Magenta,
        Color::Cyan
    ];

    // Predetermined hidden combination
    let predetermined_guess = vec![
        Color::Red,
        Color::Red,
        Color::Green,
        Color::Green,
        Color::Green
    ];
        
    // Random combination to find
    let mut guess: Vec<Color> = vec![];
    for _ in 0..5 {
        let rng = rand::thread_rng().gen_range(0..7);
        match rng {
            0 => guess.push(Color::Red),
            1 => guess.push(Color::Green),
            2 => guess.push(Color::Blue),
            3 => guess.push(Color::Yellow),
            4 => guess.push(Color::Orange),
            5 => guess.push(Color::Purple),
            6 => guess.push(Color::Magenta),
            7 => guess.push(Color::Cyan),
            _ => {}
        }
    }

    println!("Welcome in the game of MasterMind ! Give a combination composed of 5 colors:\nR => Red;\nG => Green;\nB => Blue;\nY => Yellow;\nO => Orange;\nP => Purple;\nM => Magenta;\nC => Cyan\n");

    // Print combination by letter
    // fancy_print_guess(&guess);

    // Number of tries user currently attempted
    let mut tries = 1;

    // Infinite loop reading user input
    for user_input in io::stdin().lock().lines() {
        let input = user_input.unwrap();

        // You can cheat but it's bad :)
        if input == "cheat" {
            fancy_print_guess(&guess);
        } else {
            // Transform input in Vec<Color>
            let user_guess = parse_user_input(input);
            // If parse_user_input doesn't panic it's a valid input so we add 1 to number of tries
            tries +=1;
    
            print!("{} - you guessed: ", tries);
            fancy_print_guess(&user_guess);
    
            if user_guess.eq(&guess) {
                println!("CONGRATULATION, you've guessed the right combination ! You are the MasterMind !");
                break;
            } else {
                let well_placed_colors = number_of_well_placed_pawns(&guess, &user_guess);
                let not_well_placed_colors = number_of_not_well_placed_pawns(&guess, &user_guess);
                println!("Almost ! You have well placed {} colors out of the {} colors you've guessed right !\n", well_placed_colors, not_well_placed_colors);
            }
        }
    }
}
