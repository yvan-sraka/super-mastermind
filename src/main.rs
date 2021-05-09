use ansi_term::Colour::{Red, Blue, Yellow, Green, White, Om, Black, Purple};
use std::str::FromStr;
use rand::prelude::*;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum Color {
    R,
    B,
    Y,
    G,
    W,
    O,
    L,
    P,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Color, Self::Err> {
        match s {
            "R" => Ok(Color::R),
            "B" => Ok(Color::B),
            "Y" => Ok(Color::Y),
            "G" => Ok(Color::G),
            "W" => Ok(Color::W),
            "O" => Ok(Color::O),
            "L" => Ok(Color::L),
            "P" => Ok(Color::P),
            _ => Err(()),
        }
    }
}


fn ansi_color(capital_letter: &str, color: &Color) -> String {
    match color {
        Color::R => Red.paint(capital_letter).to_string(),
        Color::B => Blue.paint(capital_letter).to_string(),
        Color::Y => Yellow.paint(capital_letter).to_string(),
        Color::G => Green.paint(capital_letter).to_string(),
        Color::W => White.paint(capital_letter).to_string(),
        Color::O => Orange.paint(capital_letter).to_string(),
        Color::L=> Black.paint(capital_letter).to_string(),
        Color::P => Purple.paint(capital_letter).to_string(),
    }
}
fn fancy_print_guess(guess: &[Color]) {
    let mut jeu = String::from("");

    for elem in guess {
        match elem {
            Color::R => jeu.push_str(&ansi_color("R", &Color::R)),
            Color::B => jeu.push_str(&ansi_color("B", &Color::B)),
            Color::Y => jeu.push_str(&ansi_color("Y", &Color::Y)),
            Color::G => jeu.push_str(&ansi_color("G", &Color::G)),
            Color::W => jeu.push_str(&ansi_color("W", &Color::W)),
            Color::O => jeu.push_str(&ansi_color("O", &Color::O)),
            Color::L => jeu.push_str(&ansi_color("L", &Color::L)),
            Color::P => jeu.push_str(&ansi_color("P", &Color::P)),
        }
    }

    println!("{}", jeu)
}

fn input() -> String {
    let mut jeu = String::new();
    std::io::stdin().read_line(&mut jeu).unwrap();

    jeu
}

fn parse_jeu(joue: String) -> Vec<Color> {
    let mut combinaison1: Vec<Color> = Vec::new();
    let mut comb: Vec<&str> = joue.trim().split("").collect();
    comb = comb.into_iter().filter(|&s| s != "").collect();
    for s in comb {
        combinaison1.push(Color::from_str(s.trim()).unwrap());
    }
    return combinaison1;
    
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut count: i32 = 0;
    for i in 0..secret.len() {
        if secret[i] == guess[i]{
            count += 1
        }
    }
    count
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut count: i32 = 0;
    for i in 0..secret.len() {
        if secret[i] != guess[i]{
            count += 1
        }
    }
    count
}

fn results() -> Vec<Color> {
    let mut m = rand::thread_rng();
    let mut c: Vec<Color> = Vec::new();
    for _ in 0..5 {
        let number = m.gen_m(0..7);
        match number {
            0 => c.push(Color::R),
            1 => c.push(Color::B),
            2 => c.push(Color::Y),
            3 => c.push(Color::G),
            4 => c.push(Color::W),
            5 => c.push(Color::O),
            6 => c.push(Color::L),
            7 => c.push(Color::P),
            _ => {}
        }
    }

    return c;
}

fn main() {
    let guess = results();
    let mut number_guess= 1;
    let mut play = true;

    while play == true {
    
        println!("Le nombre est {}", number_guess);
        let played = input();

        if played.clone().trim().len() != 5 {
            continue;
        } 

        number_guess += 1;


        fancy_print_guess(&parse_jeu(played.clone()));
        if parse_jeu(played.clone()) == guess {
            println!("You win after {} guess CONGRATULATIONS!!.", number_guess);
            play = false;
            continue;
        }

        println!("you have {} colors  correctly placed", number_of_well_placed_pawns(&guess, &parse_jeu(played.clone())));
        println!("you have {} colors  not correctly placed", number_of_not_well_placed_pawns(&guess, &parse_jeu(played)));
    }
}