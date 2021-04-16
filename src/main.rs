use ansi_term::Colour;

fn main() {
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
        // let attribute = value.GetAttributeOrDefault<DisplayAttribute>();
        // println!("{:?}", guess);
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

    let guess:Vec<Color> = vec![Color::Cyan,Color::Green,Color::Black,Color::Yellow,Color::Purple];
    // loop {
    fancy_print_guess(&guess);
    fancy_print_guess(&[Color::Red, Color::Purple, Color::Black, Color::Yellow, Color::Green])
    // }
}
