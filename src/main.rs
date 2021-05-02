use ansi_term::Colour;
use std::io;
use std::io::BufRead;
use rand::Rng;

#[derive(Debug, PartialEq)]
enum Color {
    Cyan,
    Green,
    Blue,
    Purple,
    Red,
    White,
    Yellow,
}

fn fancy_print_guess(guess: &[Color]) {
    for color in guess {
        match color {
            Color::Yellow => print!("{}", Colour::Yellow.paint("Y")),
            Color::Cyan => print!("{}", Colour::Cyan.paint("C")),
            Color::Green => print!("{}", Colour::Green.paint("G")),
            Color::Blue => print!("{}", Colour::Blue.paint("B")),
            Color::Purple => print!("{}", Colour::Purple.paint("P")),
            Color::Red => print!("{}", Colour::Red.paint("R")),
            Color::White => print!("{}", Colour::White.paint("W")),
        }
    }
    println!();
}

fn parse_vec_into_string(guess: &[Color]) -> String {
    let mut result = String::new();
    for color in guess {
        match color {
            Color::Yellow => result.push('Y'),
            Color::Cyan => result.push('C'),
            Color::Green => result.push('G'),
            Color::Blue => result.push('B'),
            Color::Purple => result.push('P'),
            Color::Red => result.push('R'),
            Color::White => result.push('W'),
        }
    }
    return result;
}

fn parse_string_into_vec(input: String) -> Vec<Color>{
    let mut colors:Vec<Color> = vec![];
    for aa in input.chars() {
        match aa {
            'Y' => colors.push(Color::Yellow),
            'C' => colors.push(Color::Cyan),
            'G' => colors.push(Color::Green),
            'B' => colors.push(Color::Blue),
            'P' => colors.push(Color::Purple),
            'R' => colors.push(Color::Red),
            'W' => colors.push(Color::White),
            '\n' => print!(""),
            _ => {
                println!("Mauvaise saisie");
                return vec![];
            }
        }
    }

fancy_print_guess(&colors);
    return colors;
}
// Level 11
fn number_of_well_placed_pawns(secret: &[Color], guessColor: &[Color]) -> i32 {
    return secret.iter().zip(guessColor.iter()).filter(|&(guess, guessColor)| guess == guessColor).count() as i32;
}
// Level 12
fn number_of_not_well_placed_pawns(secret: &[Color], guessColor: &[Color]) -> i32 {
    return guessColor.iter().filter(|color| secret.contains(color)).count() as i32;
}

// Level 13
fn gencolor() -> Vec<Color> {
    let mut guess: Vec<Color> = vec![];
    for _ in 0..5 {
        let rng = rand::thread_rng().gen_range(0..6);
        match rng {
            0 => guess.push(Color::Yellow),
            1 => guess.push(Color::Cyan),
            2 => guess.push(Color::Green),
            3 => guess.push(Color::Blue),
            4 => guess.push(Color::Purple),
            5 => guess.push(Color::Red),
            6 => guess.push(Color::White),
            _ => {}
        }
    }
    guess
}

fn main() {

    println!("Bienvenue sur mon Mastermind");
    let guess= gencolor();
    // Level 10
    for user_input in io::stdin().lock().lines() {
        let input = user_input.unwrap();
        let guessColor = parse_string_into_vec(input);
        fancy_print_guess(&guessColor);
        if guessColor.eq(&guess) {
            println!("Gagné !");
            break;
        } else {
            let well_placed_colors = number_of_well_placed_pawns(&guess, &guessColor);
            let not_well_placed_colors = number_of_not_well_placed_pawns(&guess, &guessColor);
            println!("{} bien placé", well_placed_colors);
            println!("{} mal placé!", not_well_placed_colors)
        }
    }
}
