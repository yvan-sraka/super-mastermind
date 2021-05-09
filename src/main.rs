use ansi_term::Colour;


fn main() {

    let guess = vec![
        Color::Black,
        Color::White,
        Color::Yellow,
        Color::Blue,
        Color::Purple
    ];
    fancy_print_guess(&guess);
}


enum Color{
    Black, White, Yellow, Blue, Purple, Green, Orange, Grey, Red
}

fn fancy_print_guess(guess: &[Color]) {
    for c in guess {
        match c {
            Color::Black => print!("{}", Colour::RGB(0,0,0).paint("B")),
            Color::White => print!("{}", Colour::RGB(255,255,255).paint("W")),
            Color::Yellow => print!("{}", Colour::RGB(255,240,0).paint("Y")),
            Color::Blue => print!("{}", Colour::RGB(0,0,255).paint("B")),
            Color::Purple => print!("{}", Colour::RGB(255,0,255).paint("P")),
            Color::Green => print!("{}", Colour::RGB(0,255,0).paint("G")),
            Color::Orange => print!("{}", Colour::RGB(255,100,0).paint("O")),
            Color::Grey => print!("{}", Colour::RGB(150,150,150).paint("G")),
            Color::Red => print!("{}", Colour::RGB(255,0,0).paint("R")),
        }
    }
}
