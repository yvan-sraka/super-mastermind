enum Color {
    Red,
    Pink,
    Blue,
    Black,
    Green,
    White,
    Purple,
    Yellow
}

fn main() {
    let mut guess: Vec<Color> = Vec::new();
    guess.push(Color::Black);
    guess.push(Color::Blue);
    guess.push(Color::Green);
    guess.push(Color::Black);
    guess.push(Color::Black);
}
