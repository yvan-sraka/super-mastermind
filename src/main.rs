use core::fmt;
use std::io::Write;
use ansi_term::Colour;
use rand::prelude::*;
use std::ptr::null;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Blue,
    Purple,
    Yellow,
    Black,
    Green,
    Cyan,
    White,
}

fn get_random_colors() -> Vec<Color> {
    let mut rng = rand::thread_rng();

    let mut colors: Vec<Color> = Vec::new();

    for i in 0..5 {
        let random_number = rng.gen_range(0..7);
        match random_number {
            0 => colors.push(Color::Black),
            1 => colors.push(Color::Blue),
            2 => colors.push(Color::Red),
            3 => colors.push(Color::Yellow),
            4 => colors.push(Color::Cyan),
            5 => colors.push(Color::Green),
            6 => colors.push(Color::Purple),
            7 => colors.push(Color::White),
            _ => {}
        }
    }

    return colors;
}

fn main() {
    let mut count = 0;

    let mut guess = get_random_colors();

    let mut play = 1;

    while play == 1 {
        print!("Enter your string : ");
        std::io::stdout().flush();
        let mut string: String = String::new();
        std::io::stdin().read_line(&mut string);
        let parse_input = parse_input(string.trim().parse().unwrap());
        fancy_print_guess(&*parse_input);

        if compare_colors_vec(&*guess, &*parse_input) {
            play = 0;
            println!("You won ! CONGRATULATIONS 🥳🎉");
        } else {
            count += 1;
            if count >= 5 {
                play = 0;
                println!("YOU LOST 🙃, it was :");
                fancy_print_guess(&*guess);
            } else {
                println!("Wrong input, try again !");
                println!("Number of right inputs : {}", number_of_well_placed_pawns(&*guess, &*parse_input));
                println!("Number of wrong inputs : {}", number_of_not_well_placed_pawns(&*guess, &*parse_input));
            }
        }
    }
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    return guess.len() as i32 - number_of_well_placed_pawns(secret, guess);
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut count = 0;

    for i in 0..secret.len() {
        if guess.len() > i && secret[i] == guess[i] {
            count += 1;
        }
    }

    return count;
}

fn parse_input(s: String) -> Vec<Color> {
    let mut colors = Vec::new();

    for c in s.chars() {
        match c {
            'B' => colors.push(Color::Black),
            'b' => colors.push(Color::Blue),
            'R' => colors.push(Color::Red),
            'Y' => colors.push(Color::Yellow),
            'C' => colors.push(Color::Cyan),
            'G' => colors.push(Color::Green),
            'P' => colors.push(Color::Purple),
            'W' => colors.push(Color::White),
            _ => colors.push(Color::White)
        }
    }

    return colors;
}

fn compare_colors_vec(vec1: &[Color], vec2: &[Color]) -> bool {
    if vec2.len() != vec1.len() {
        return false;
    }

    for i in 0..vec1.len() {
        if vec2.len() < i || vec1[i] != vec2[i] {
            return false;
        }
    }

    return true;
}

fn fancy_print_guess(guess: &[Color]) {
    let mut colors = String::from("");

    for color in guess {
        match color {
            Color::Red => print!("{}", Colour::Red.paint("R")),
            Color::Blue => print!("{}", Colour::Blue.paint("B")),
            Color::Black => print!("{}", Colour::Black.paint("B")),
            Color::Yellow => print!("{}", Colour::Yellow.paint("Y")),
            Color::Cyan => print!("{}", Colour::Cyan.paint("C")),
            Color::Green => print!("{}", Colour::Green.paint("G")),
            Color::Purple => print!("{}", Colour::Purple.paint("P")),
            Color::White => print!("{}", Colour::White.paint("W")),
        }
    }

    println!("{}", colors);
}
