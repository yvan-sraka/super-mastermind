#[derive(Debug)]
enum Color {
    Red,
    Orange,
    Cyan,
    Black,
    Green,
    White,
    Purple,
    Yellow
}

fn fancy_print_guess(guess: &[Color]) {
    for color in guess {
        match color {
            Color::Red => print!("R"),
            Color::Orange => print!("O"),
            Color::Cyan => print!("C"),
            Color::Black => print!("B"),
            Color::Green => print!("G"),
            Color::White => print!("W"),
            Color::Purple => print!("P"),
            Color::Yellow => print!("Y"),
            _ => println!("Color not found"),
        }
    }
    println!("");
}

fn main() {
    let mut guess: Vec<Color> = Vec::new();
    guess.push(Color::Black);
    guess.push(Color::Orange);
    guess.push(Color::Green);
    guess.push(Color::Cyan);
    guess.push(Color::Black);

    fancy_print_guess(&guess);
}
