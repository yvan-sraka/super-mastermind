use ansi_term::Colour::{Blue, Cyan, Green, Purple, Red, White, Yellow, RGB};
use ansi_term::{ANSIString, ANSIStrings};
use rand::prelude::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum Color {
    White,
    Orange,
    Blue,
    Green,
    Red,
    Yellow,
    Purple,
    Cyan,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn color_first_color_letter(color: &Color) -> ANSIString {
    let first_letter = color.to_string().chars().next().unwrap().to_string();
    return match *color {
        Color::Blue => Blue.paint(first_letter),
        Color::Orange => RGB(255, 165, 0).paint(first_letter),
        Color::Cyan => Cyan.paint(first_letter),
        Color::Green => Green.paint(first_letter),
        Color::Purple => Purple.paint(first_letter),
        Color::Red => Red.paint(first_letter),
        Color::White => White.paint(first_letter),
        Color::Yellow => Yellow.paint(first_letter),
    };
}

fn fancy_print_guess(guess: &[Color]) {
    let mut vec_colors: Vec<ANSIString> = Vec::new();

    for color in guess {
        vec_colors.push(color_first_color_letter(color));
    }
    println!("{}", ANSIStrings(&vec_colors));
}

fn map_str_color_to_enum(str_color: &str) -> Result<Color, String> {
    match str_color.to_lowercase().as_str() {
        "white" => Ok(Color::White),
        "orange" => Ok(Color::Orange),
        "blue" => Ok(Color::Blue),
        "green" => Ok(Color::Green),
        "red" => Ok(Color::Red),
        "yellow" => Ok(Color::Yellow),
        "purple" => Ok(Color::Purple),
        "cyan" => Ok(Color::Cyan),
        _ => Err(str_color.to_lowercase()),
    }
}

fn parse_str_to_vec_color(str_colors: String) -> Result<Vec<Color>, String> {
    let mut vec_colors: Vec<Color> = Vec::new();
    let mut maybe_error = None;

    for color in str_colors.trim().split(" ") {
        match map_str_color_to_enum(color) {
            Ok(result_color) => vec_colors.push(result_color),
            Err(err_str_color) => {
                maybe_error = Some(format!(
                    "color with name {} not found. Retry :",
                    err_str_color
                ));
            }
        }
    }
    match maybe_error {
        Some(error) => Err(error),
        None => Ok(vec_colors),
    }
}

fn get_str_colors_stdin() -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    return s;
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut number_not_well_placed = 0;
    let mut index_not_take_account = Vec::<usize>::new();

    for (i, _guess_color) in guess.iter().enumerate() {
        if _guess_color == &secret[i] {
            index_not_take_account.push(i);
        }
    }

    for (i, _guess_color) in guess.iter().enumerate() {
        for (j, _secret_color) in secret.iter().enumerate() {
            if i != j && _guess_color == _secret_color && !index_not_take_account.contains(&j) {
                number_not_well_placed = number_not_well_placed + 1;
                index_not_take_account.push(j);
                break;
            }
        }
    }

    return number_not_well_placed;
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut number_well_placed = 0;
    for (i, _color) in guess.iter().enumerate() {
        if _color == &secret[i] {
            number_well_placed = number_well_placed + 1;
        }
    }
    return number_well_placed;
}

fn compare_current_colors_to_secret_colors(
    current_colors: Vec<Color>,
    secret_colors: Vec<Color>,
) -> bool {
    let number_well_placed_pawns = number_of_well_placed_pawns(&secret_colors, &current_colors);
    if number_well_placed_pawns as usize == secret_colors.len() {
        return true;
    }
    println!("number_well_placed_pawns : {}", number_well_placed_pawns);
    let number_not_well_placed_pawns =
        number_of_not_well_placed_pawns(&secret_colors, &current_colors);
    println!(
        "number not well placed pawns : {}",
        number_not_well_placed_pawns
    );
    return false;
}

fn start_game(secret_colors: Vec<Color>) {
    let mut number_try = 1;
    loop {
        print!("List of colors :");
        fancy_print_guess(&get_all_color_vec());
        println!(
            "Please enter {} fullname colors (example 'Blue') : ",
            secret_colors.len()
        );
        let str_colors = get_str_colors_stdin();
        let result = parse_str_to_vec_color(str_colors);
        match result {
            Ok(vec_color) => {
                fancy_print_guess(&vec_color);
                if compare_current_colors_to_secret_colors(vec_color, secret_colors.clone()) {
                    break;
                }
                number_try = number_try + 1;
            }
            Err(err_message) => println!("{}", err_message),
        }
        println!("");
    }
    println!(
        "Congrat ! You figure out the secret colors with {} try !",
        number_try
    );
}

fn get_all_color_vec() -> Vec<Color> {
    let mut color_vec = Vec::<Color>::new();
    color_vec.push(Color::Blue);
    color_vec.push(Color::Cyan);
    color_vec.push(Color::Green);
    color_vec.push(Color::Orange);
    color_vec.push(Color::Purple);
    color_vec.push(Color::Red);
    color_vec.push(Color::White);
    color_vec.push(Color::Yellow);
    return color_vec;
}

fn create_secret_colors(mut number_secret_colors: i32) -> Vec<Color> {
    println!("number_secret_colors : {}", number_secret_colors);
    let mut secret_colors = Vec::<Color>::new();
    let mut rng = rand::thread_rng();
    let color_vec = get_all_color_vec();

    let len_color_vec = color_vec.len() - 1;
    let mut numbers: Vec<usize> = (0..len_color_vec).collect();

    loop {
        if number_secret_colors == 0 {
            break;
        }
        numbers.shuffle(&mut rng);
        secret_colors.push(color_vec[numbers[0]].clone());
        number_secret_colors = number_secret_colors - 1;
    }

    return secret_colors;
}

fn prepare_secret_colors() -> Vec<Color> {
    loop {
        println!("Select number of secret colors to figure out.");
        match get_str_colors_stdin().trim().parse::<i32>() {
            Ok(number_secret_colors) => {
                if number_secret_colors > 0 {
                    return create_secret_colors(number_secret_colors);
                }
                println!("You have to select number of secret colors greater than 0");
            }
            Err(err_message) => println!(
                "Not correct number, enter integer greater than 0. Error message : {}",
                err_message
            ),
        }
        println!("");
    }
}

fn is_game_continue() -> bool {
    loop {
        println!("Continue ? Y / N");
        let response = get_str_colors_stdin();
        match response.trim() {
            "Y" => return true,
            "N" => return false,
            _ => println!("Please enter Y or N letter"),
        }
    }
}

fn main() {
    loop {
        let secret_colors = prepare_secret_colors();
        start_game(secret_colors);
        if !is_game_continue() {
            break;
        }
        println!("");
    }
    println!("Bye!")
}

#[cfg(test)]
mod tests {
    use super::number_of_not_well_placed_pawns;
    use super::Color;

    #[test]
    fn not_well_placed_pawns_are_not_well_placed_pawns() {
        let secret_colors = vec![Color::Blue, Color::Green, Color::White];
        let guess_colors = vec![Color::Blue, Color::Green, Color::White];
        let result = number_of_not_well_placed_pawns(&secret_colors, &guess_colors);
        assert_eq!(0, result);
    }

    #[test]
    fn when_guess_colors_contain_one_color_on_secret_colors_but_not_well_placed_should_return_1() {
        let secret_colors = vec![Color::Blue, Color::Green, Color::White];
        let guess_colors = vec![Color::Orange, Color::Blue, Color::Yellow];
        let result = number_of_not_well_placed_pawns(&secret_colors, &guess_colors);
        assert_eq!(1, result);
    }

    #[test]
    fn when_guess_colors_contain_3_colors_on_secret_colors_but_not_well_placed_should_return_3() {
        let secret_colors = vec![Color::Blue, Color::Green, Color::White];
        let guess_colors = vec![Color::Green, Color::White, Color::Blue];
        let result = number_of_not_well_placed_pawns(&secret_colors, &guess_colors);
        assert_eq!(3, result);
    }

    #[test]
    fn when_guess_colors_not_contain_colors_on_secret_colors_should_return_0() {
        let secret_colors = vec![Color::Blue, Color::Green, Color::White];
        let guess_colors = vec![Color::Yellow, Color::Purple, Color::Orange];
        let result = number_of_not_well_placed_pawns(&secret_colors, &guess_colors);
        assert_eq!(0, result);
    }

    #[test]
    fn when_guess_colors_contain_2_same_colors_of_only_one_on_secret_colors_should_return_1() {
        let secret_colors = vec![Color::Blue, Color::Green, Color::White];
        let guess_colors = vec![Color::Green, Color::Purple, Color::Green];
        let result = number_of_not_well_placed_pawns(&secret_colors, &guess_colors);
        assert_eq!(1, result);
    }

    #[test]
    fn when_guess_colors_contain_2_same_colors_of_2_on_secret_colors_should_return_2() {
        let secret_colors = vec![Color::Blue, Color::Green, Color::White, Color::Green];
        let guess_colors = vec![Color::Green, Color::Purple, Color::Green, Color::Purple];
        let result = number_of_not_well_placed_pawns(&secret_colors, &guess_colors);
        assert_eq!(2, result);
    }

    #[test]
    fn when_guess_colors_contains_2_well_placed_colors_and_2_not_well_placed_colors_should_return_2(
    ) {
        let secret_colors = vec![Color::White, Color::Cyan, Color::Red, Color::Red];
        let guess_colors = vec![Color::White, Color::Red, Color::Cyan, Color::Red];
        let result = number_of_not_well_placed_pawns(&secret_colors, &guess_colors);
        assert_eq!(2, result);
    }

    #[test]
    fn when_guess_colors_is_full_same_colors_and_secret_have_2_of_this_color_should_return_0() {
        let secret_colors = vec![Color::White, Color::Cyan, Color::White, Color::Red];
        let guess_colors = vec![Color::White, Color::White, Color::White, Color::White];
        let result = number_of_not_well_placed_pawns(&secret_colors, &guess_colors);
        assert_eq!(0, result);
    }
}
