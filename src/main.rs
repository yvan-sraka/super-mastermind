use ansi_term::Colour;
use std::io::Write;

#[derive(Debug)]
enum Color {
    Red,
    Black,
    Cyan,
    Blue,
    Green,
    White,
    Purple,
    Yellow
}

fn fancy_print_guess(guess: &[Color]) {
    for color in guess {
        match color {
            Color::Red => print!("{}", Colour::Red.paint("R")),
            Color::Black => print!("{}", Colour::Black.paint("B")),
            Color::Cyan => print!("{}", Colour::Cyan.paint("C")),
            Color::Blue => print!("{}", Colour::Blue.paint("b")),
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
    let mut guess: Vec<Color> = Vec::new();
    guess.push(Color::Red);
    guess.push(Color::Black);
    guess.push(Color::Green);
    guess.push(Color::Cyan);
    guess.push(Color::Blue);
    let mut game = true;

    while game == true {
        let mut input: String = String::new();
        
        print!("Enter your asnwer:");

        std::io::stdout().flush();
        std::io::stdin().read_line(&mut input);
        
        input = String::from(input.trim());
        
        if input.len() != 5 {
            println!("/!\\ You need to input only 5 colors (letters) /!\\");
        } else {
            fancy_print_guess(&guess);
            game = false;
        }
    }
}
