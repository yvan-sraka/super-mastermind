use std::io::{self, Read};
use ansi_term::Colour;
use rand::prelude::*;

#[derive(PartialEq)]
#[derive(Debug)]
enum Color{
    RED,
    BLUE,
    GREEN,
    ORANGE,
    YELLOW,
    PINK,
    CYAN,
    WHITE,
}

fn generate_secret() -> Vec<Color> {
    let mut secret = vec![];
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (1..9).collect();
    let y: f64 = rng.gen(); // generates a float between 0 and 1

    nums.shuffle(&mut rng);

    for n in (0..5) {
        match nums[n] {
            1 => secret.push(Color::RED),
            2 => secret.push(Color::BLUE),
            3 => secret.push(Color::GREEN),
            4 => secret.push(Color::ORANGE),
            5 => secret.push(Color::YELLOW),
            6 => secret.push(Color::PINK),
            7 => secret.push(Color::CYAN),
            8 => secret.push(Color::WHITE),
            _ => secret.push(Color::WHITE),
        }
    }

    secret
}

fn print_congratulation() {
    println!("🎈 Congratulation 🎈")
}

fn fancy_print_guess(guess: &[Color]) {

    for color in guess {
        match color {
            Color::RED => print!("{}", Colour::Red.paint("R")),
            Color::BLUE => print!("{}", Colour::Blue.paint("B")),
            Color::GREEN => print!("{}", Colour::Green.paint("G")),
            Color::ORANGE => print!("{}", Colour::RGB(255, 165, 0).paint("O")),
            Color::YELLOW => print!("{}", Colour::Yellow.paint("Y")),
            Color::PINK => print!("{}", Colour::RGB(255, 20, 147).paint("P")),
            Color::CYAN => print!("{}", Colour::Cyan.paint("C")),
            Color::WHITE => print!("W"),
        }
    }
    println!();
}

fn parse_string_into_colors(mut input: String) -> Result<Vec<Color>, String> {
    let mut _guess: Vec<Color> = vec![];
    input.truncate(input.len()-1);

    if input.len() > 5 {
        return Err("Your input is too long".to_string());
    } else if input.len() < 5 {
        return Err("Your input is too short".to_string());
    } else if input.len() == 0 {
        return Err("You must type at least 5 characters".to_string());
    } 

    for c in input.chars() {
        match c {
            'R' => _guess.push(Color::RED),
            'B' => _guess.push(Color::BLUE),
            'G' => _guess.push(Color::GREEN),
            'O' => _guess.push(Color::ORANGE),
            'Y' => _guess.push(Color::YELLOW),
            'P' => _guess.push(Color::PINK),
            'C' => _guess.push(Color::CYAN),
            'W' => _guess.push(Color::WHITE),
            _ => return Err("You must use R, B, G, O, Y, P, C or W".to_string()),
        }
    }
    
    Ok(_guess)
}

fn is_code_equals_guess(guess: &[Color], code: &[Color]) -> bool {
    for (pos, e) in guess.iter().enumerate() {
        if guess[pos] != code[pos] {
            return false;
        }
    }
    return true;
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut result = 0;
    for (pos, e) in guess.iter().enumerate() {
        if guess[pos] == secret[pos] {
            result += 1;
        }
    }
    return result;
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    5 - number_of_well_placed_pawns(secret, guess)
}

fn welcome() {
    println!("Welcome to the mastermind game 🧠");
    println!("Allowed colors are : R, B, G, O, Y, P, C or W")
}

fn main() {
    welcome();
    let mut input = String::new();
    let stdin = io::stdin();
    let mut try_count: i32 = 0;
    let mut well_placed: i32 = 0; 
    //let mut _code: Vec<Color> = vec![Color::BLUE, Color::RED, Color::CYAN, Color::YELLOW, Color::PINK];
    let mut _code: Vec<Color> = generate_secret();
    let mut _guess: Vec<Color> = vec![Color::BLUE];

    while well_placed != 5 {        
        stdin.read_line(&mut input);
        println!("{}", input);

        _guess = match parse_string_into_colors(input){
            Ok(guess) =>{
                try_count += 1;
                guess
            },
            Err(e) => {
                println!("{}", e);
                Vec::new()
            }, 
        };
        fancy_print_guess(&_guess);

        well_placed = number_of_well_placed_pawns(&_code, &_guess);
        println!("You placed well {} paws", well_placed);
        println!("You get {} not well placed pawns", number_of_not_well_placed_pawns(&_code, &_guess));

        input = String::from("");
        println!("You made {} try", try_count);
        println!();
    }

    print_congratulation();
}
