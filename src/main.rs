#![allow(unused)]
#[warn(dead_code)]

use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use ansi_term::Colour;
use rand::prelude::*;

#[derive(Debug)]
#[derive(PartialEq)]
enum Color {
    R,
    G,
    B,
    P,
    Y,
    W,
    O,
}

#[derive(Debug)]
enum Error {
    ParseInputError,
}

fn main() {
    
    let mut to_find = Vec::<Color>::new();
    let mut rng = thread_rng();

    for i in 0..=4 {
        let val = match rng.gen_range(0..=6) { // rand 0.8
            0 => Color::R,
            1 => Color::G,
            2 => Color::B,
            3 => Color::P,
            4 => Color::Y,
            5 => Color::W,
            _ => Color::O
        };
        to_find.push(val);
    }

    println!("Welcome to the SUPER MASTER MIND!");


    let mut retry_counter = 1;
    loop {
        let mut s = String::new();
        println!("Enter your combination: try n°{}", retry_counter);
        let _=stdout().flush();
        stdin().read_line(&mut s);

        
        let parsed_guesses: Vec<Color> = match parse_input(s) {
            Ok(parsed)  => parsed,
            Err(e) => {
                println!("   Results:");
                println!("{}", Colour::Red.paint("      Wrong inputs use only R, G, B, P, Y, W, O"));
                Vec::new()
            },
        };
        if parsed_guesses.len() > 0 {
            fancy_print_guess(&parsed_guesses);
            println!("   Results:");
            println!("      Well placed pawns: {}", number_of_well_placed_pawns(&to_find, &parsed_guesses));
            println!("      Non well placed pawns: {}", number_of_not_well_placed_pawns(&to_find, &parsed_guesses));

            if number_of_well_placed_pawns(&to_find, &parsed_guesses) == to_find.len() as i32 {
            println!("{}", Colour::Green.paint("                                                             
                                                           _       
                                                          | |      
                            ___ ___  _ __   __ _ _ __ __ _| |_ ___ 
                           / __/ _ \\| '_ \\ / _` | '__/ _` | __/ __|
                          | (_| (_) | | | | (_| | | | (_| | |_\\__ \\ 
                           \\___\\___/|_| |_|\\__, |_|  \\__,_|\\__|___/
                                            __/ |                  
                                           |___/    
                                                                                "));
                println!("{}", Colour::Green.paint("                          🎈 You made it 🎈, this is the correct combo!"));
                fancy_print_guess(&to_find);

                break
            }
        }
        retry_counter += 1;
    }
}

fn number_of_well_placed_pawns(secret: &Vec<Color>, guesses: &[Color]) -> i32 {
    let mut counter = 0;
    for (i,guess) in guesses.iter().enumerate() {
        if *guess == secret[i] {
            counter += 1;
        }
    }   
    counter
}

fn number_of_not_well_placed_pawns(secret: &Vec<Color>, guesses: &Vec<Color>) -> i32 {
    let mut counter = 0;
    for (i,guess) in guesses.iter().enumerate() {
        if *guess == secret[i] {
            continue
        }
        let mut is_color_in_other_place = false;
        for (j,secret_color) in secret.iter().enumerate() {
        
            if *guess == *secret_color && i != j {
                is_color_in_other_place = true;
            }
        }
        if is_color_in_other_place {
            counter += 1;
        }
    }   
    counter
}

fn parse_input(mut input: String) -> Result<Vec<Color>, Error> {
    input.truncate(input.len()-1);
    let mut vec = Vec::<Color>::new();
    for c in input.chars() {
        match c {
            'R' => vec.push(Color::R),
            'G' => vec.push(Color::G),
            'B' => vec.push(Color::B),
            'P' => vec.push(Color::P),
            'Y' => vec.push(Color::Y),
            'O' => vec.push(Color::O),
            'W' => vec.push(Color::W),
            _ => return Err(Error::ParseInputError),
        }
    }

    Ok(vec)
}

fn fancy_print_guess(guesses: &[Color]) {
    let mut print = Vec::new();
    
    for guess in guesses {
        match guess {
            Color::R => print.push(Colour::Red.paint("R")),
            Color::G => print.push(Colour::Green.paint("G")),
            Color::B => print.push(Colour::Blue.paint("B")),
            Color::P => print.push(Colour::Purple.paint("P")),
            Color::O => print.push(Colour::RGB(255, 105, 0).paint("O")),
            Color::Y => print.push(Colour::Yellow.paint("Y")),
            Color::W => print.push(Colour::White.paint("W")),
        }
    }
    for p in print {
        print!("{}", p);
    }
    println!("");
}
