use std::io::Write;
use rand::prelude::*;
use ansi_term::Colour;


#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Cyan,
    Blue,
    Black,
    Green,
    White,
    Purple,
    Yellow,
}

fn get_random_colors() -> Vec<Color> {
    let mut rng = rand::thread_rng();
    let mut secret: Vec<Color> = Vec::new();

    for i in 0..5 {
        let random_nb = rng.gen_range(0..7);
        match random_nb {
            0 => secret.push(Color::Red),
            1 => secret.push(Color::Cyan),
            2 => secret.push(Color::Blue),
            3 => secret.push(Color::Black),
            4 => secret.push(Color::Green),
            5 => secret.push(Color::White),
            6 => secret.push(Color::Purple),
            7 => secret.push(Color::Yellow),
            _ => (),
        }
    }

    return secret;
}

fn number_of_well_placed_pawns(secret: &[Color], answer: &[Color]) -> i32 {
    let mut well_placed = 0;
    for (index, _) in answer.iter().enumerate() {
        if answer[index] == secret[index] {
            well_placed = well_placed + 1;
        }
    }
    return well_placed; 
}

fn number_of_not_well_placed_pawns(secret: &[Color], answer: &[Color]) -> i32 {
    let mut not_well_placed = 0;
    let mut well_placed_table = [0;5];

    for (i,x) in answer.iter().enumerate() {
        if answer[i] == secret[i] {
            well_placed_table[i] = 1;
        }
    }
    
    for (si,sx) in secret.iter().enumerate() {
        if well_placed_table[si] == 0 {
            for (gi,gx) in answer.iter().enumerate() {
                if well_placed_table[gi] == 0 && si != gi && sx == gx {
                    not_well_placed = not_well_placed + 1;
                }
            }
        }

    }
    return not_well_placed;
}

fn user_answer(input: String) -> Vec<Color> {
    let mut user_answer: Vec<Color> = Vec::new(); 
    for letter in input.chars() {
        match letter {
            'R' => user_answer.push(Color::Red),
            'C' => user_answer.push(Color::Cyan),
            'b' => user_answer.push(Color::Blue),
            'W' => user_answer.push(Color::White),
            'B' => user_answer.push(Color::Black),
            'G' => user_answer.push(Color::Green),
            'Y' => user_answer.push(Color::Yellow),
            'P' => user_answer.push(Color::Purple),
            _ => println!("Color not found with {}", letter),
        }
    }
    return user_answer;
}

fn is_answer(guess: &[Color], input: &[Color]) -> bool {
    for (index, _) in guess.iter().enumerate() {
        if guess[index] != input[index] {
            return false;
        }
    }
    return true;
}

fn fancy_print_guess(guess: &[Color]) {
    for color in guess {
        match color {
            Color::Red => print!("{}", Colour::Red.paint("R")),
            Color::Cyan => print!("{}", Colour::Cyan.paint("C")),
            Color::Blue => print!("{}", Colour::Blue.paint("b")),
            Color::Black => print!("{}", Colour::Black.paint("B")),
            Color::Green => print!("{}", Colour::Green.paint("G")),
            Color::White => print!("{}", Colour::White.paint("W")),
            Color::Purple => print!("{}", Colour::Purple.paint("P")),
            Color::Yellow => print!("{}", Colour::Yellow.paint("Y")),
            _ => println!("Color not found"),
        }
    }
    println!("");
}

fn main() {
    let mut game = true;
    let mut times_played = 0;
    let mut secret: Vec<Color> = get_random_colors();
    
    while game == true {
        let mut input: String = String::new();
        
        print!("Enter your answer:");

        std::io::stdout().flush();
        std::io::stdin().read_line(&mut input);
        
        input = String::from(input.trim());
        
        if input.len() != 5 {
            println!("input 5 colors ... /!\\");
            continue;
        }
        let answer: Vec<Color> = user_answer(input);
        
        times_played = times_played + 1;

        fancy_print_guess(&answer);
        
        if is_answer(&secret, &answer) {
            println!("You found the good code with : {} tries", times_played);
            game = false;
        } else {
            println!("colors well placed : {}", number_of_well_placed_pawns(&secret, &answer));
            println!("And have {} colors not well placed !", number_of_not_well_placed_pawns(&secret, &answer));
        }
    }
}
