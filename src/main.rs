use ansi_term::Colour;
use std::io;

fn main() {
    let mut guess: Vec<Color> = Vec::new();
    let mut vec: Vec<Color> = Vec::new();
    guess.push(Color::Red);
    guess.push(Color::Blue);
    guess.push(Color::Green);
    guess.push(Color::Yellow);
    guess.push(Color::Purple);
    guess.push(Color::White);
    guess.push(Color::Cyan);
    guess.push(Color::Maroon);

    //rintln!("{:?}", Color::Maroon);

    print_vec(guess);

    //println!("{}", Colour::RGB(102, 51, 0).paint("M"));
    fancy_print_guess(&[Color::Red, Color::Red, Color::Blue, Color::Yellow, Color::Green, Color::Maroon]);

    let mut input = String::new();
    loop{
        let error = io::stdin().read_line(&mut input);
        // match input.as_str(){
        //     "R" => vec.push(Color::Red),
        //     _ => println!("Got some other input")
        // }
        for c in input.chars() {
            match c{
                'R' => vec.to_owned().push(Color::Red),
                _ => println!("{}", c)
            }
            //print!("{}", c);
        }
        //println!("{}", input);
        input = String::from("");
        println!("error: {:?}", error);
        print_vec(vec);
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