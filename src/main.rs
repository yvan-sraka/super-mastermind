use ansi_term::Colour;
use std::io::Write;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Cyan,
    Blue,
    Black,
    Green,
    White,
    Purple,
    Yellow,
}

fn user_answer(input: String) -> Vec<Color> {
    let mut user_answer: Vec<Color> = Vec::new(); 
    for letter in input.chars() {
        match letter {
            'R' => user_answer.push(Color::Red),
            'C' => user_answer.push(Color::Cyan),
            'b' => user_answer.push(Color::Blue),
            'W' => user_answer.push(Color::White),
            'B' => user_answer.push(Color::Black),
            'G' => user_answer.push(Color::Green),
            'Y' => user_answer.push(Color::Yellow),
            'P' => user_answer.push(Color::Purple),
            _ => println!("Color not found with {}", letter),
        }
    }
    return user_answer;
}

fn is_answer(guess: &[Color], input: &[Color]) -> bool {
    for (index, _) in guess.iter().enumerate() {
        if guess[index] != input[index] {
            return false;
        }
    }
    return true;
}

fn fancy_print_guess(guess: &[Color]) {
    for color in guess {
        match color {
            Color::Red => print!("{}", Colour::Red.paint("R")),
            Color::Cyan => print!("{}", Colour::Cyan.paint("C")),
            Color::Blue => print!("{}", Colour::Blue.paint("b")),
            Color::Black => print!("{}", Colour::Black.paint("B")),
            Color::Green => print!("{}", Colour::Green.paint("G")),
            Color::White => print!("{}", Colour::White.paint("W")),
            Color::Purple => print!("{}", Colour::Purple.paint("P")),
            Color::Yellow => print!("{}", Colour::Yellow.paint("Y")),
            _ => println!("Color not found"),
        }
    }
    println!("");
}

fn main() {
    let mut game = true;
    let mut times_played = 0;
    let mut guess: Vec<Color> = Vec::new();
    guess.push(Color::Red);
    guess.push(Color::Black);
    guess.push(Color::Green);
    guess.push(Color::Cyan);
    guess.push(Color::Blue);

    while game == true {
        let mut input: String = String::new();
        
        print!("Enter your asnwer:");

        std::io::stdout().flush();
        std::io::stdin().read_line(&mut input);
        
        input = String::from(input.trim());
        
        if input.len() != 5 {
            println!("/!\\ You need to input only 5 colors (letters) /!\\");
            continue;
        }
        let answer: Vec<Color> = user_answer(input);
        
        times_played = times_played + 1;

        fancy_print_guess(&answer);
        
        if is_answer(&guess, &answer) {
            println!("you found the good code with : {} tries", times_played);
            game = false;
        }
    }
}
