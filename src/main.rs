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

        match x {
            Color::Yellow => s.push_str(&Yellow.paint("Y").to_string()),
            Color::Cyan => s.push_str(&Cyan.paint("C").to_string()),
            Color::Red => s.push_str(&Red.paint("R").to_string()),
            Color::Green => s.push_str(&Green.paint("G").to_string()),
            Color::Blue => s.push_str(&Blue.paint("B").to_string()),
            Color::Purple => s.push_str(&Purple.paint("P").to_string()),
            //rgb(255,0,255)
            Color::Magenta => s.push_str(&Colour::RGB(255,0,255).paint("M").to_string()),
            //rgb(255,165,0)
            Color::Orange => s.push_str(&Colour::RGB(255,165,0).paint("O").to_string()),
            Color::White => s.push_str(&White.paint("W").to_string()),
        }
    }

    return s;
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
