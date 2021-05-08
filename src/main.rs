use ansi_term::Colour;
use std::process;
use rand::Rng;

use strum::IntoEnumIterator; // To iterate through Enum Color
use strum_macros::EnumIter; // To iterate through Enum Color

#[derive(Debug, PartialEq, EnumIter)]
enum Color {
    Blue,
    Red,
    Green,
    Purple,
    Yellow,
    White,
    Cyan,
    Desert
}

fn main() {
    println!("Mastermind\n=====\n");

    show_color_available();

    let mut guess_number: i32 = 0;
    let mut secret: Vec<Color> = Vec::new();

    secret = generate_random_secret_colors();

    /*    print!("Secret colors : ");
        fancy_print_guess(&secret);*/

    loop {
        let mut input: String = String::new();
        println!("Find the right colors :");

        std::io::stdin().read_line(&mut input);
        input = String::from(input.trim());

        // Check if the input contains 5 characters
        if input.chars().count() == 5 {
            let guess = parse_input(input);
            fancy_print_guess(&guess);

            // Check if the return contains 5 valid characters
            if guess.iter().count() == 5 {
                guess_number = guess_number + 1;

                // Check if the return contains 5 valid characters
                if guess == secret {
                    println!("Correct guess ! Congratulation 🎉 you guessed the secret colors in {} attempt(s).", guess_number);
                    process::exit(0);
                } else {
                    println!("Pawn(s) well placed ✅ : {}", number_of_well_placed_pawns(&*guess, &*secret));
                    println!("Incorrect pawn(s) ❌: {}", number_of_not_well_placed_pawns(&*guess, &*secret));
                    println!("Number of guess : {}", guess_number);
                }
            }
        } else {
            print_error_message("Incorrect input ! Please choose 5 colors available. ex : BRGPY or DRYWC");
        }
    }
}

fn parse_input(input: String) -> Vec<Color>{
    let mut colors = Vec::new();

    for char in input.chars() {
        match char {
            'B' => {
                colors.push(Color::Blue);
            }
            'R' => {
                colors.push(Color::Red);
            }
            'G' => {
                colors.push(Color::Green);
            }
            'P' => {
                colors.push(Color::Purple);
            }
            'Y' => {
                colors.push(Color::Yellow);
            }
            'W' => {
                colors.push(Color::White);
            }
            'C' => {
                colors.push(Color::Cyan);
            }
            'D' => {
                colors.push(Color::Desert);
            }
            _ => {
                let message = format!("Incorrect character [{}]", char);
                print_error_message(&*message);
            }
        }
    }

    return colors;
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut well_placed_pawns = 0;

    for (i, _) in secret.iter().enumerate() {
        if secret[i] == guess[i] {
            well_placed_pawns = well_placed_pawns + 1;
        }
    }

    return well_placed_pawns;
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    return 5 - number_of_well_placed_pawns(secret, guess);
}

fn generate_random_secret_colors() -> Vec<Color> {
    let mut random_colors: Vec<Color> = Vec::new();

    for _ in 0..5 {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..7);
        match random_number {
            0 => {
                random_colors.push(Color::Blue);
            }
            1 => {
                random_colors.push(Color::Red);
            }
            2 => {
                random_colors.push(Color::Green);
            }
            3 => {
                random_colors.push(Color::Purple);
            }
            4 => {
                random_colors.push(Color::Yellow);
            }
            5 => {
                random_colors.push(Color::White);
            }
            6 => {
                random_colors.push(Color::Cyan);
            }
            7 => {
                random_colors.push(Color::Desert);
            }
            _ => {}
        }
    }

    return random_colors;
}

fn show_color_available() -> () {
    let mut colors: Vec<Color> = Vec::new();

    for color in Color::iter() {
        colors.push(color);
    }

    print!("Colors available : ");
    fancy_print_guess(&colors);
}

fn fancy_print_guess(guess: &[Color]) {
    for color in guess {
        match color {
            Color::Blue => {
                print!("{}", Colour::Blue.paint("B"));
            }
            Color::Red => {
                print!("{}", Colour::Red.paint("R"));
            }
            Color::Green => {
                print!("{}", Colour::Green.paint("G"));
            }
            Color::Purple => {
                print!("{}", Colour::Purple.paint("P"));
            }
            Color::Yellow => {
                print!("{}", Colour::RGB(255, 255, 0).paint("Y"));
            }
            Color::White => {
                print!("{}", Colour::RGB(255, 255, 255).paint("W"));
            }
            Color::Cyan => {
                print!("{}", Colour::Cyan.paint("C"));
            }
            Color::Desert => {
                print!("{}", Colour::RGB(139, 69, 19).paint("D"));
            }
        }
    }
    print!("\n");
}

fn print_error_message(message: &str) {
    println!("{}", Colour::RGB(255, 50, 50).paint(message));
}
