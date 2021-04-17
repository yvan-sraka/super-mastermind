use rand::*;
use std::io::{self};

#[derive(Debug, PartialEq)]
enum Color {
    Blue,
    Cyan,
    Red,
    Green,
    Yellow,
    Purple,
}

fn fancy_print_guess(letters_to_paint: &[Color]) {
    use ansi_term::Colour::{Blue, Cyan, Green, Purple, Red, Yellow};
    for c in letters_to_paint {
        match c {
            &Color::Blue => print!("{}", Blue.paint("B ")),
            &Color::Cyan => print!("{}", Cyan.paint("C ")),
            &Color::Green => print!("{}", Green.paint("G ")),
            &Color::Yellow => print!("{}", Yellow.paint("Y ")),
            &Color::Purple => print!("{}", Purple.paint("P ")),
            &Color::Red => print!("{}", Red.paint("R ")),
        }
    }
    println!()
}

fn count_colors_found(secret: &Vec<Color>, guess: &Vec<Color>) -> i32 {
    let mut matching = 0;
    for c in secret {
        if guess.contains(&c) {
            matching += 1;
        }
    }
    matching
}

fn count_well_placed_colors(secret: &Vec<Color>, guess: &Vec<Color>) -> i32 {
    let mut matching = 0;
    if secret.len() != guess.len() {
        return matching;
    }
    for i in 0..secret.len() {
        if secret[i] == guess[i] {
            matching += 1;
        }
    }
    matching
}

fn get_all_colors_to_guess() -> Vec<Color> {
    let all_colors: Vec<Color> = vec![
        Color::Blue,
        Color::Cyan,
        Color::Green,
        Color::Yellow,
        Color::Purple,
        Color::Red,
    ];
    all_colors
}

fn construct_colors_to_guess() -> Vec<Color> {
    let mut guess = Vec::<Color>::new();
    let mut rng = thread_rng(); // https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum

    for _c in 0..6 {
        let temp = match rng.gen_range(0..6) {
            0 => Color::Blue,
            1 => Color::Cyan,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Purple,
            _ => Color::Red,
        };
        guess.push(temp);
    }
    guess
}

fn give_result(guessed: Vec<Color>, secret: &Vec<Color>) -> i32 {
    let found_colors = count_colors_found(&secret, &guessed);
    let well_placed_colors = count_well_placed_colors(&secret, &guessed);
    if guessed.len() == secret.len() {
        println!("colors found : {}", found_colors);
        println!("well placed : {}", well_placed_colors);
    }
    well_placed_colors
}

fn print_congrats(secret: &Vec<Color>) {
    use ansi_term::Colour::Blue;
    println!("{}", Blue.paint("Congratulations you found it!!!"));
    fancy_print_guess(&secret);
}

fn main() {
    let secret = construct_colors_to_guess();

    let mut first = true;
    loop {
        if first {
            print!("Welcome! ");
            first = false;
        }
        print!("Please enter a combination of 6 letters from those : ");
        fancy_print_guess(&get_all_colors_to_guess());
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                let mut ok = true;
                let mut guessed = Vec::new();

                for i in 0..n - 1 {
                    match input.chars().nth(i).unwrap() {
                        'B' => guessed.push(Color::Blue),
                        'R' => guessed.push(Color::Red),
                        'C' => guessed.push(Color::Cyan),
                        'G' => guessed.push(Color::Green),
                        'Y' => guessed.push(Color::Yellow),
                        'P' => guessed.push(Color::Purple),
                        _ => {
                            println!("There is a wrong letter, please enter a new combination");
                            ok = false;
                        }
                    }
                    if !ok {
                        break;
                    }
                }
                if ok {
                    if guessed.len() == give_result(guessed, &secret) as usize {
                        print_congrats(&secret);
                        break;
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
