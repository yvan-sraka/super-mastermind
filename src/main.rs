use ansi_term::Colour::{Blue, Cyan, Green, Purple, Red, White, Yellow, RGB};
use ansi_term::{ANSIString, ANSIStrings};
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
    let mut index_not_well_placed = Vec::<usize>::new();

    for (i, _guess_color) in guess.iter().enumerate() {
        for (j, _secret_color) in secret.iter().enumerate() {
            if i != j
                && _guess_color == _secret_color
                && &secret[i] != _guess_color
                && !index_not_well_placed.contains(&j)
            {
                number_not_well_placed = number_not_well_placed + 1;
                index_not_well_placed.push(j);
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
    let mut result = true;
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
    for (i, _color) in current_colors.iter().enumerate() {
        if secret_colors[i] != *_color {
            result = false;
        }
    }
    return result;
}

fn start_game(secret_colors: Vec<Color>) {
    let mut number_try = 1;
    loop {
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
        println!("\n");
    }
    println!(
        "Congrat ! You figure out the secret colors with {} try !",
        number_try
    );
}

fn main() {
    let secret_colors = vec![Color::Blue, Color::Green, Color::White];
    start_game(secret_colors);
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
}
