mod colors;

use crate::colors::colors::Color;
use ansi_term::Colour;
use std::fmt;
use std::io::stdout;
use std::io::{self};


const GUESS_SIZE: usize = 5;

#[derive(Debug, Clone)]
struct ParseColorError {
    color_char: char
}
impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} isn't a known Color enum value", self.color_char)
    }
}

// struct BadInputSize;
// impl fmt::Display for BadInputSize {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{} isn't a known Color enum value", self.color_char)
//     }
// }



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
        _ => Err(ParseColorError { color_char: character })
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

fn get_guesses_result(user_colors: Vec<Color>, guesses: Vec<Color>) -> (u8, u8) {
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

fn main() {
    println!("Hello, world!");
    let guess = vec![
        Color::PURPLE,
        Color::RED,
        Color::GREEN,
        Color::ORANGE,
        Color::YELLOW,
        // Color::BLUE,
        // Color::WHITE,
        // Color::CYAN
    ];

    println!("simple print");
    println!("{:?}", guess);
    println!("fancy print");
    fancy_print_guess(&guess);

    let mut game_over = false;

    let mut turn = 0;

    while !game_over {
        turn += 1;

        let user_input = read_line();

        let user_colors: Vec<Color> = input_string_to_color_guess(&user_input).unwrap();

        println!("{}", user_input);

        let guesses: (u8, u8) = get_guesses_result(user_colors.to_vec(), guess.to_vec());

        if guesses.0 == 5 {
            game_over = true;
        } else {
            print_turn_result(guesses);
        }
    }

    greeting_player(turn);
}
