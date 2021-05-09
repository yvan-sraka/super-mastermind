use ansi_term::Colour::{Fixed, White, Yellow, Red, Purple, Blue, Green, Cyan};
use rand::prelude::*;

#[warn(dead_code)]
#[derive(Debug, PartialEq)]
enum Color {
    Mustard,
    White,
    Yellow,
    Red,
    Purple,
    Blue,
    Green,
    Cyan
}

fn fancy_print_guess(guess: &Vec<Color>) {
    let mut output = String::new();
    let mut format_color;
    for color in guess {
        format_color = format!("{:?}", color);
        match color {
            Color::Mustard =>  print!("{}", Fixed(221).paint(&format_color[..1])),
            Color::White =>  print!("{}", White.paint(&format_color[..1])),
            Color::Yellow => print!("{}", Yellow.paint(&format_color[..1])),
            Color::Red => print!("{}", Red.paint(&format_color[..1])),
            Color::Purple => print!("{}", Purple.paint(&format_color[..1])),
            Color::Blue => print!("{}", Blue.paint(&format_color[..1])),
            Color::Green => print!("{}", Green.paint(&format_color[..1])),
            Color::Cyan => print!("{}", Cyan.paint(&format_color[..1])),
        }
    }
    println!();
}

fn make_answer(line: String) -> Vec<Color> {
    let mut answer :Vec<Color> = Vec::with_capacity(5);
    for character in line.chars() {
        match character {
            'M' => answer.push(Color::Mustard),
            'W' => answer.push(Color::White),
            'Y' => answer.push(Color::Yellow),
            'R' => answer.push(Color::Red),
            'P' => answer.push(Color::Purple),
            'B' => answer.push(Color::Blue),
            'G' => answer.push(Color::Green),
            'C' => answer.push(Color::Cyan),
            other => println!("Error, {} doesn't represent any color here.", other)
        }
    }
    answer
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut number = 0;
    for count in 0..5 {
        if secret[count] == guess[count] {
            number += 1;
        }
    }
    number
}

fn randomly_combination() -> Vec<Color> {
    let mut secret: Vec<Color> = vec![];
    let mut rng = rand::thread_rng();
    for number in 0..5 {
        let y: i32 = rng.gen_range(0..7);
        match y {
            0 => secret.push(Color::Mustard),
            1 => secret.push(Color::White),
            2 => secret.push(Color::Yellow),
            3 => secret.push(Color::Red),
            4 => secret.push(Color::Purple),
            5 => secret.push(Color::Blue),
            6 => secret.push(Color::Green),
            7 => secret.push(Color::Cyan),
            _ => {}
        }
    }
    secret
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut number = 0;
    for count in 0..5 {
        if secret.contains(&guess[count]) && secret[count] != guess[count] {
            number += 1;
        }
    }
    number
}


fn main() {
    let secret = randomly_combination();
    let guess: Vec<Color> = vec![];
    //println!("{}", number_of_well_placed_pawns(&*secret, &*guess));
    //println!("{}", number_of_not_well_placed_pawns(&*secret, &*guess));
/*
    loop {
        let mut input =String::new();
        std::io::stdin().read_line(&mut input);
        input = String::from(input.trim());
        guess = make_answer(input);



    }

 */
}

