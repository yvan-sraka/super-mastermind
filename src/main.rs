use ansi_term::Colour::{Black, White, Yellow, Red, Purple, Blue, Green, Cyan};

#[warn(dead_code)]
#[derive(Debug)]
enum Color {
    Black,
    White,
    Yellow,
    Red,
    Purple,
    Blue,
    Green,
    Cyan
}

fn fancy_print_guess(guess: &[Color]) {
    let mut output = String::new();
    let mut format_color;
    for color in guess {
        format_color = format!("{:?}", color);
        match color {
            Color::Black => { print!("{}", Black.paint(&format_color[..1])); }
            Color::White => { print!("{}", White.paint(&format_color[..1])); }
            Color::Yellow => { print!("{}", Yellow.paint(&format_color[..1])); }
            Color::Red => { print!("{}", Red.paint(&format_color[..1])); }
            Color::Purple => { print!("{}", Purple.paint(&format_color[..1])); }
            Color::Blue => { print!("{}", Blue.paint(&format_color[..1])); }
            Color::Green => { print!("{}", Green.paint(&format_color[..1])); }
            Color::Cyan => { print!("{}", Cyan.paint(&format_color[..1])); }
        }
    }
    println!();
}

fn main() {
    let guess = [Color::Blue, Color::Yellow, Color::Red, Color::Green, Color::Purple];
    fancy_print_guess(&guess);
    // reading_and_printing_loop();
}

fn reading_and_printing_loop(guess :&[Color]) {
    loop {
        let mut line =String::new();
        std::io::stdin().read_line(&mut line);

        println!("{}", line);
    }
}
