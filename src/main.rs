use ansi_term::Colour;
use std::io;

fn main() {
    // let mut vec: Vec<Color> = Vec::new();
    // guess.push(Color::Red);
    // guess.push(Color::Blue);
    // guess.push(Color::Green);
    // guess.push(Color::Yellow);
    // guess.push(Color::Purple);
    // guess.push(Color::White);
    // guess.push(Color::Cyan);
    // guess.push(Color::Maroon);

    //rintln!("{:?}", Color::Maroon);

    // print_vec(guess);

    //println!("{}", Colour::RGB(102, 51, 0).paint("M"));
    fancy_print_guess(&[Color::Red, Color::Red, Color::Blue, Color::Yellow, Color::Green, Color::Maroon]);

    let mut input = String::new();
    loop{
        let mut guess = vec![];
        io::stdin().read_line(&mut input);
        println!("Input : {}", input);
        let letters = input.trim();
        for letter in letters.chars(){
            let color = match letter {
                'R' => Color::Red,
                'B' => Color::Blue,
                'G' => Color::Green,
                'Y' => Color::Yellow,
                'P' => Color::Purple,
                'W' => Color::White,
                'C' => Color::Cyan,
                'M' => Color::Maroon,
                _ => { 
                    println!("Color {} is invalid", letter);
                    break;
                }
            };
            guess.push(color);
        }
        print_vec(guess);
    }
}

#[derive(Debug)]
enum Color{
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
    White,
    Cyan,
    Maroon
}

fn print_vec(guess: Vec<Color>){
    for color in guess {
        println!("{:?}", color);
    }
}

fn fancy_print_guess(guess: &[Color]){
    let mut vec = Vec::new();
    for color in guess {
        match color{
            Color::Red => vec.push(Colour::Red.paint("R")),
            Color::Blue => vec.push(Colour::Blue.paint("B")),
            Color::Green => vec.push(Colour::Green.paint("G")),
            Color::Yellow => vec.push(Colour::Yellow.paint("Y")),
            Color::Purple => vec.push(Colour::Purple.paint("P")),
            Color::White => vec.push(Colour::White.paint("W")),
            Color::Cyan => vec.push(Colour::Cyan.paint("I")),
            Color::Maroon => vec.push(Colour::Fixed(89).paint("M"))
        }
    }
    for el in vec {
        print!("{}", el);
    }
    println!();
}