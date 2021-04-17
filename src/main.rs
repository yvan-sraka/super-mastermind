use ansi_term::Colour;
use std::io;
use std::fs::File;
use std::io::{Seek, SeekFrom, Read};

fn main() -> io::Result<()> {
    #[derive(Debug, PartialEq)]
    enum Color {
        Cyan,
        Green,
        Blue,
        Black,
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
                Color::Black => print!("{}", Colour::Black.paint("M")),
                Color::Blue => print!("{}", Colour::Blue.paint("O")),
                Color::Purple => print!("{}", Colour::Purple.paint("P")),
                Color::Red => print!("{}", Colour::Red.paint("R")),
                Color::White => print!("{}", Colour::White.paint("W")),
            }
        }
    }

    fn factory_print_guess(proposal: String){

    }


    let guess:Vec<Color> = vec![Color::Cyan,Color::Green,Color::Black,Color::Yellow,Color::Purple];



    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    factory_print_guess(input);


    // fancy_print_guess(&guess);
    // fancy_print_guess(&[Color::Red, Color::Purple, Color::Black, Color::Yellow, Color::Green]);

/*    loop {
        fancy_print_guess(&guess);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim() == "CGBYP"{
            break;
        }
    }*/
    Ok(())

}
