use ansi_term::Colour::{self, Blue, Cyan, Green, Purple, Red, RGB, White, Yellow};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::io;

#[allow(non_upper_case_globals)]
const Orange: Colour = RGB(255, 165, 0);

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Purple,
    Orange,
    Yellow,
    White,
    Cyan,
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0..=7) {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            3 => Color::Purple,
            4 => Color::Orange,
            5 => Color::Yellow,
            6 => Color::White,
            _ => Color::Cyan,
        }
    }
}



fn fancy_print_guess(guessed_color: &[Color]) {
    for c in guessed_color {
        match c {
            &Color::Blue => print!("{}", Blue.paint("B ")),
            &Color::Cyan => print!("{}", Cyan.paint("C ")),
            &Color::Green => print!("{}", Green.paint("G ")),
            &Color::Yellow => print!("{}", Yellow.paint("Y ")),
            &Color::Purple => print!("{}", Purple.paint("P ")),
            &Color::Red => print!("{}", Red.paint("R ")),
            &Color::Orange => print!("{}", Orange.paint("O ")),
            &Color::White => print!("{}", White.paint("W ")),
        }
    }
    println!()
}

// L11
fn number_of_well_placed_pawns(secret_colors: &Vec<Color>, guessed_color: &Vec<Color>) -> i32 {
    let mut matching = 0;
    if secret_colors.len() != guessed_color.len() {
        return matching;
    }
    for i in 0..secret_colors.len() {
        if secret_colors[i] == guessed_color[i] {
            matching += 1;
        }
    }
    matching
}

// L12
fn number_of_not_well_placed_pawns(secret_colors: &Vec<Color>, guessed_color: &Vec<Color>) -> i32 {
    (secret_colors.len() as i32) - number_of_well_placed_pawns(secret_colors, guessed_color)
}


fn print_colors() {
    println!("({}) {}", Red.bold().paint("R"), Red.bold().paint("Red"));
    println!("({}) {}", Green.bold().paint("G"), Green.bold().paint("Green"));
    println!("({}) {}", Blue.bold().paint("B"), Blue.bold().paint("Blue"));
    println!("({}) {}", Cyan.bold().paint("C"), Cyan.bold().paint("Cyan"));
    println!("({}) {}", Yellow.bold().paint("Y"), Yellow.bold().paint("Yellow"));
    println!("({}) {}", Orange.bold().paint("O"), Orange.bold().paint("Orange"));
    println!("({}) {}", White.bold().paint("W"), White.bold().paint("White"));
    println!("({}) {}", Purple.bold().paint("P"), Purple.bold().paint("Purple"));
    println!();
}

// L13
fn color_generator() -> Vec<Color> {
    let mut secret_colors: Vec<Color> = vec![];
    for _ in 0..6 {
        let rng = rand::thread_rng().gen_range(0..7);
        match rng {
            0 => secret_colors.push(Color::Red),
            1 => secret_colors.push(Color::Green),
            2 => secret_colors.push(Color::Blue),
            3 => secret_colors.push(Color::Purple),
            4 => secret_colors.push(Color::Orange),
            5 => secret_colors.push(Color::Yellow),
            6 => secret_colors.push(Color::White),
            7 => secret_colors.push(Color::Cyan),
            _ => {}
        }
   }
   secret_colors
}

fn main() -> Result<(), io::Error> {
    let secret_colors: Vec<Color> = color_generator();
    let mut nb_tries = 1;

    println!("Welcome to Mastermind, can you guessed_color the {} color combination ?", secret_colors.len());
    println!("Supported colors are:'\n");
    print_colors();

    // L8
    loop {
        let mut guessed_color = vec![];
        let mut input = String::new();

        print!("Enter your guessed_color: ");
        io::stdin().read_line(&mut input)?;

        let input_colors = input.trim();

        for letter in input_colors.chars() {
            let letter_uppercase = letter.to_ascii_uppercase();
            let color = match letter_uppercase {
                'R' => Color::Red,
                'G' => Color::Green,
                'B' => Color::Blue,
                'P' => Color::Purple,
                'O' => Color::Orange,
                'Y' => Color::Yellow,
                'W' => Color::White,
                'C' => Color::Cyan,
                _ => {
                    println!("The letter '{}' you entered is not a valid Color, try again", letter);
                    print_colors();

                    break;
                }
            };

            guessed_color.push(color);
        }

        if input_colors.len() != secret_colors.len() {
            println!("Not enough colors entered. Minimum colors is {}", secret_colors.len());
            continue;
        }

        if guessed_color.len() != secret_colors.len() {
            continue;
        }

        fancy_print_guess(&guessed_color);
        let well_placed_pawns = number_of_well_placed_pawns(&secret_colors, &guessed_color);
        let not_well_placed_pawns = number_of_not_well_placed_pawns(&secret_colors, &guessed_color);

        if guessed_color == secret_colors {
            break;
        }

        
        print!("try number {}: ", nb_tries);
        println!("{} well_placed_pawns, {} not_well_placed_pawns", well_placed_pawns, not_well_placed_pawns);
        println!();

        nb_tries +=1;
    }

    println!("Congratulation! You killed the game ");

    Ok(())
}