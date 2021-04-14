//Write your own function fancy_print_guess(guess: &[Color]) to display combination (for e.g. represent each color with a different capital letter):
mod color;
use color::Color;

// fancy_print_guess(&[Color::Red, Color::Red, Color::Blue, Color::Yellow, Color::Green]);
//ouput `RRBYG`
fn fancy_print_guess(guess: &[Color]) -> String{
    let mut s = String::new();
    for x in guess {
        println!("test vector : {:?}", x);

        match x {
            Color::Yellow => s.push('Y'),
            Color::Cyan => s.push('C'),
            Color::Red => s.push('R'),
            Color::Green => s.push('G'),
            Color::Blue => s.push('B'),
            Color::Purple => s.push('P'),
            Color::Magenta => s.push('M'),
            Color::Orange => s.push('O'),
            Color::White => s.push('W'),
        }
    }
    return s;
}

fn main() {
    println!("test enum {:?}", Color::Yellow);

    let mut guess:Vec<Color> = Vec::new();
    guess.push(Color::Yellow);
    guess.push(Color::Cyan);
    guess.push(Color::Red);
    guess.push(Color::Green);
    guess.push(Color::Blue);

    println!("test vector : {:?}", guess[1]);
    let color_to_guess = fancy_print_guess(&guess);
    println!("colors to guess : {:?}", color_to_guess);
}
