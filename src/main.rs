//Write your own function fancy_print_guess(guess: &[Color]) to display combination (for e.g. represent each color with a different capital letter):
mod color;
use color::Color;
use ansi_term::{Style, Colour::*};
use ansi_term::Colour;

// fancy_print_guess(&[Color::Red, Color::Red, Color::Blue, Color::Yellow, Color::Green]);
//ouput `RRBYG`
fn fancy_print_guess(guess: &[Color]) -> String{
    let mut s = String::new();
    for x in guess {
        println!("test vector : {:?}", x);
        let mut letterColored = fancy_color_letter(x);
        s.push_str(&letterColored);
    }
    return s;
}

fn fancy_color_letter(color: &Color) -> String{
    match color {
        Color::Yellow => return Yellow.paint("Y").to_string(),
        Color::Cyan => return Cyan.paint("C").to_string(),
        Color::Red => return Red.paint("R").to_string(),
        Color::Green => return Green.paint("G").to_string(),
        Color::Blue => return Blue.paint("B").to_string(),
        Color::Purple => return Purple.paint("P").to_string(),
        //rgb(255,0,255)
        Color::Magenta => return Colour::RGB(255,0,255).paint("M").to_string(),
        //rgb(255,165,0)
        Color::Orange => return Colour::RGB(255,165,0).paint("O").to_string(),
        Color::White => return White.paint("W").to_string(),
    };
}

fn main() {
    println!("test enum {:?}", Color::Yellow);

    //générer automatiquement
    let mut guess:Vec<Color> = Vec::new();
    guess.push(Color::Yellow);
    guess.push(Color::Cyan);
    guess.push(Color::Red);
    guess.push(Color::Green);
    guess.push(Color::Blue);

    println!("test vector : {:?}", guess[1]);
    let color_to_guess = fancy_print_guess(&guess);
    println!("colors to guess : {}", color_to_guess);

    //faire une boucle qui demande une combinaison de lettres
    //Make an infinite loop which at each turn: read a string and print it!


}
