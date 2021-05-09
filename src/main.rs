#[derive(Debug)]
enum Color {
    Blue,
    Yellow,
    Red,
    Orange,
    Grey,
    White,
    Purple,
    Navy
}

fn fancy_print_guess(guess: &[Color]) {
    let mut output = String::from("");
    for color in guess {
        let format_color = format!("{:?}", color);
        output.push_str(&format_color[..1]);
    }
    println!("{:?}", output)
}

fn main() {
    let guess = [Color::Blue, Color::Yellow, Color::Red, Color::Grey, Color::Purple];
    fancy_print_guess(&guess);
}
