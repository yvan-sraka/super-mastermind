extern crate rand;

use std::fmt;
use std::io::{self};
use ansi_term::Colour;
use rand::Rng;

// 0.4.2
use crate::colors::colors::Color;

mod colors;
// 0.8.0

const GUESS_SIZE: usize = 5;

#[derive(Debug, Clone)]
struct ParseColorError {
    color_char: char,
    message: String,
}
impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} isn't a known Color enum value", self.color_char)
    }
}

struct BadInputSize {
    waited_size: usize,
    item_size: usize,
}

impl fmt::Display for BadInputSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is not a valid size. item of length {} waited.", self.item_size, self.waited_size)
    }
}

fn character_to_color(character: char) -> Result<Color, ParseColorError> {
    match character {
        'B' => Ok(Color::BLUE),
        'R' => Ok(Color::RED),
        'P' => Ok(Color::PURPLE),
        'G' => Ok(Color::GREEN),
        'Y' => Ok(Color::YELLOW),
        'O' => Ok(Color::ORANGE),
        'C' => Ok(Color::CYAN),
        'W' => Ok(Color::WHITE),
        _ => Err(ParseColorError { message: "Undefined Character".to_string(), color_char: character })
    }
}

fn get_random_color() -> Color {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=8) {
        0 => Color::BLUE,
        1 => Color::RED,
        2 => Color::PURPLE,
        3 => Color::GREEN,
        4 => Color::YELLOW,
        5 => Color::ORANGE,
        6 => Color::CYAN,
        _ => Color::WHITE
    }
}

fn get_ansi_colour_from_color(color: &Color) -> Colour {
    match color {
        Color::BLUE => Colour::Blue,
        Color::RED => Colour::Red,
        Color::PURPLE => Colour::Purple,
        Color::GREEN => Colour::Green,
        Color::YELLOW => Colour::RGB(250, 190, 0),
        Color::ORANGE => Colour::RGB(255, 110, 0),
        Color::CYAN => Colour::Cyan,
        Color::WHITE => Colour::White,
    }
}

fn fancy_print_guess(guess: &[Color]) {
    for c in guess {
        let ansi_colour = get_ansi_colour_from_color(c);
        print!("{}", ansi_colour.paint(c.to_string()));
    }
    println!()
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");


    return input[..input.len() - 1].to_string();
}

fn input_string_to_color_guess(input: &str) -> Result<Vec<Color>, ParseColorError> {
    let mut ret = Vec::new();
    for character in input.chars() {
        ret.push(character_to_color(character)?);
    }
    return Ok(ret);
}

fn get_guesses_result(user_colors: &Vec<Color>, guesses: &Vec<Color>) -> (u8, u8) {
    let mut matches = 0;
    let mut misplaced = 0;
    let mut well_placed_index: [bool; GUESS_SIZE] = [false; GUESS_SIZE];

    for i in 0..user_colors.len() {
        if user_colors[i] == guesses[i] {
            matches += 1;
            well_placed_index[i] = true;
        }
    }

    for i in 0..user_colors.len() {
        if !well_placed_index[i] {
            for j in 0..guesses.len() {
                if !well_placed_index[j] && user_colors[i] == guesses[j] {
                    misplaced += 1;
                    break;
                }
            }
        }
    }

    return (matches, misplaced);
}

fn greeting_player(turn: u8) {
    println!("BRAVO ! Vous avez trouvé en {} tour(s)", turn);
}

fn print_turn_result(p0: (u8, u8)) {
    println!("{} bien placé(s)", p0.0);
    println!("{} mal placé(s)", p0.1);
}

fn print_turn_starting_info() {
    println!("Tentez de trouver la combinaison de {} couleurs !", GUESS_SIZE);
}

fn main() {
    let guess = vec![
        get_random_color(),
        get_random_color(),
        get_random_color(),
        get_random_color(),
        get_random_color(),
    ];

    println!("Une sélection aléatoire de {} couleurs aléatoires a été générées", GUESS_SIZE);


    let mut game_over = false;
    let mut turn = 0;

    while !game_over {
        print_turn_starting_info();
        turn += 1;

        let user_input = read_line();

        if user_input.len() != 5 {
            println!("{}", BadInputSize { item_size: user_input.len(), waited_size: 5})
        } else {
            println!("{}", user_input);

            let user_colors: Result<Vec<Color>, ParseColorError> = input_string_to_color_guess(&user_input);

            if user_colors.is_err() {

                println!("{}", user_colors.unwrap_err());

            } else {
                let user_colors_value = user_colors.unwrap();
                fancy_print_guess(&user_colors_value);

                let guesses: (u8, u8) = get_guesses_result(&user_colors_value, &guess.to_vec());

                if guesses.0 == 5 {
                    game_over = true;
                } else {
                    print_turn_result(guesses);
                }
            }
        }
    }

    greeting_player(turn);
}
