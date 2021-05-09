use ansi_term::Colour;
use std::io::{self, Read};
use rand::prelude::*;

#[derive(PartialEq)]
#[derive(Debug)]
enum Color {
    RED,
    BLUE,
    GREEN,
    YELLOW,
    CYAN,
    DESERT,
    PURPLE,
    ORANGE
}

fn fancy_print_guess(guess: &[Color]) {
    for color in guess {
        match color {
            Color::RED => print!("{}", Colour::Red.paint("R")),
            Color::BLUE => print!("{}", Colour::Blue.paint("B")),
            Color::GREEN => print!("{}", Colour::Green.paint("G")),
            Color::YELLOW => print!("{}", Colour::Yellow.paint("Y")),
            Color::CYAN => print!("{}", Colour::Cyan.paint("C")),
            Color::DESERT => print!("{}", Colour::RGB(193,154,107).paint("D")),
            Color::PURPLE => print!("{}", Colour::Purple.paint("P")),
            Color::ORANGE => print!("{}", Colour::RGB(255,127,80).paint("O"))
        }
    }
    println!();
}


fn generate_secret_code() -> Vec<Color> {
    let mut secret = vec![];
    
    let mut rng = rand::thread_rng();

    for n in (1..6) {
        let random: i32 = rng.gen_range(1..8);
        match random {
            1 => secret.push(Color::RED),
            2 => secret.push(Color::BLUE),
            3 => secret.push(Color::GREEN),
            4 => secret.push(Color::YELLOW),
            5 => secret.push(Color::CYAN),
            6 => secret.push(Color::DESERT),
            7 => secret.push(Color::PURPLE),
            _ => secret.push(Color::ORANGE),
        }
    }

    return secret;
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32{
    let mut well_placed = 0;
    for (index, color) in guess.iter().enumerate() {
        if guess[index] == secret[index] {
            well_placed += 1;
        }
    }
    return well_placed;
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut not_well_placed = 0;
    for (index, color) in guess.iter().enumerate() {
        if guess[index] != secret[index] {
            not_well_placed += 1;
        }
    }
    return not_well_placed;
}

fn convert_string_to_guess_colors(mut input: String) -> Result<Vec<Color>, String>{
    let mut guess: Vec<Color> = vec![];
    input.truncate(input.len()-1);
    
    if input.len() > 5 {
        return Err("Input is too long".to_string());
    } else if input.len() < 5 {
        return Err("Input is too short".to_string());
    }

    for color in input.chars() {
        match color {
            'R' => guess.push(Color::RED),
            'B' => guess.push(Color::BLUE),
            'G' => guess.push(Color::GREEN),
            'Y' => guess.push(Color::YELLOW),
            'C' => guess.push(Color::CYAN),
            'D' => guess.push(Color::DESERT),
            'P' => guess.push(Color::PURPLE),
            'O' => guess.push(Color::ORANGE),
            _ => return Err("No match for this color".to_string()),
        }
    }
    
    Ok(guess)
}

fn main() {
    println!("Hello Rust!");
    println!("This is a mastermind, you have to guess the color combination! Type first letter of color capitalized. ");

    // let mut guess: Vec<Color> = vec![Color::RED,Color::CYAN,Color::PURPLE,Color::GREEN,Color::DESERT];
    // fancy_print_guess(&guess);
    let mut guess: Vec<Color> = vec![];

    
    // let secret: Vec<Color> = vec![Color::DESERT,Color::BLUE,Color::BLUE,Color::PURPLE,Color::ORANGE];
    let secret: Vec<Color> = generate_secret_code();

    let max_guesses = 10;
    let mut input = String::new();
    let mut guesses = 0;
    let stdin = io::stdin();

    while guesses < max_guesses {
        stdin.read_line(&mut input);
        
        guess = match convert_string_to_guess_colors(input){
            Ok(guess) =>{
                guesses += 1;
                guess
            },
            Err(error) => {
                println!("{}", error);
                Vec::new()
            }, 
        };
        
        fancy_print_guess(&guess);
        let well_placed = number_of_well_placed_pawns(&guess, &secret);
        let not_well_placed = number_of_not_well_placed_pawns(&guess, &secret);

        if well_placed == 5 {
            println!("You did find it ! Bravo !");
            break;
        } else {
            println!("You well placed {} elements", well_placed);
            println!("You missplaced {} elements", not_well_placed);
            println!("You have {} guesses left", max_guesses - guesses);
        }

        input = String::from("");
    }
    if guesses == max_guesses {
        
        println!("You have used all your guesses...");
    }
}
