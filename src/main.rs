use ansi_term::Colour;
use std::io::Write;
use rand::Rng;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Purple,
}

fn fancy_print_guess(guess: &[Color]){
    for color in guess{
        match color {
            Color::Red => print!("{}", Colour::Red.paint("R")),
            Color::Orange => print!("{}", Colour::RGB(255, 127, 0).paint("O")),
            Color::Yellow => print!("{}", Colour::Yellow.paint("Y")),
            Color::Green => print!("{}", Colour::Green.paint("G")),
            Color::Blue => print!("{}", Colour::Blue.paint("B")),
            Color::Indigo => print!("{}", Colour::RGB(75, 0, 130).paint("I")),
            Color::Purple => print!("{}", Colour::Purple.paint("P")),
        }
    }
    println!();
}

fn string_to_vec(input: String) -> Vec<Color>{
    let mut colors: Vec<Color> = Vec::new();
    for c in input.chars(){
        match c {
            'R' => colors.push(Color::Red),
            'O' => colors.push(Color::Orange),
            'Y' => colors.push(Color::Yellow),
            'G' => colors.push(Color::Green),
            'B' => colors.push(Color::Blue),
            'I' => colors.push(Color::Indigo),
            'P' => colors.push(Color::Purple),
            _ => {
                println!("Wrong input, please use R, O, Y, G, B, I or P");
                return Vec::new();
            }
        }
    }
    return colors;
}

fn number_of_well_placed_pawns(secret: &[Color], guess2: &[Color]) -> i32 {
    return secret.iter().zip(guess2.iter()).filter(|&(guess, guess2)| guess == guess2).count() as i32;
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess2: &[Color]) -> i32 {
    return guess2.iter().filter(|color| secret.contains(color)).count() as i32;
}

fn rand_colors() -> Vec<Color>{
    let mut secret_code: Vec<Color> = Vec::new();
    for i in 0..5{
        let rng = rand::thread_rng().gen_range(0..6);
        match rng {
            0 => secret_code.push(Color::Red),
            1 => secret_code.push(Color::Orange),
            2 => secret_code.push(Color::Yellow),
            3 => secret_code.push(Color::Green),
            4 => secret_code.push(Color::Blue),
            5 => secret_code.push(Color::Indigo),
            6 => secret_code.push(Color::Purple),
            _ => {},
        }
    }
    return secret_code;
}

fn main() {
    println!("{}", Colour::White.bold().paint("Let's play Mastermind !"));
    println!("{}", Colour::White.bold().paint("-------------------------------------"));
    println!("{} {}, {}, {}, {}, {}, {} and {}", Colour::White.bold().paint("The available colors in the game are"), Colour::Red.paint("Red"), Colour::RGB(255, 127, 0).paint("Orange"), Colour::Yellow.paint("Yellow"), Colour::Green.paint("Green"), Colour::Blue.paint("Blue"), Colour::RGB(75, 0, 130).paint("Indigo"), Colour::Purple.paint("Purple"));
    println!("{} {}, {}, {}, {}, {}, {} and {} {}", Colour::White.bold().paint("Please use the letters"), Colour::Red.paint("R"), Colour::RGB(255, 127, 0).paint("O"), Colour::Yellow.paint("Y"), Colour::Green.paint("G"), Colour::Blue.paint("B"), Colour::RGB(75, 0, 130).paint("I"), Colour::Purple.paint("P"), Colour::White.bold().paint("to play"));

    let secret_code = rand_colors();
    let mut tries = 0;
    let mut win = false;

    while !win{
        println!("\nEnter the 5 colors and press enter");
        if tries == 0 {
            print!("First guess : ");
        } else {
            print!("New guess : ");
        }
        std::io::stdout().flush();
        let mut user_input: String = String::new();
        std::io::stdin().read_line(&mut user_input);
        user_input = String::from(user_input.trim());
        if user_input.len() != 5 && user_input != "code" {
            println!("You need 5 letters");
            continue;
        } else if user_input == "code" {
            println!("{:?}", &secret_code);
            println!("You cheater !")
            // break;
        }
        tries+=1;
        let user_guess = string_to_vec(user_input);
        print!("Try n°{} : ", tries);
        fancy_print_guess(&user_guess);
        if &user_guess == &secret_code {
            println!("Congratulations !! You won !");
            win = true;
            break;
        } else {
            let well_placed_colors = number_of_well_placed_pawns(&secret_code, &user_guess);
            let not_well_placed_colors = number_of_not_well_placed_pawns(&secret_code, &user_guess);
            println!("You have {} well-placed colors and {} misplaced colors", well_placed_colors, not_well_placed_colors);
        }
    }
}
