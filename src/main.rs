use ansi_term::Colour;
use std::io::Write;
use rand::prelude::*;

#[derive(Debug,PartialEq)]
enum Color {
    Blue,
    Pink,
    Orange, 
    Yellow, 
    Green, 
    White, 
    Red,
    Purple,
    None
}

fn main() {
    let mut play = true;
    let mut times = 0;

    
    let guess: Vec<Color> = get_random_colors();

    while  play == true {

        print!("Please enter some text: ");
        std::io::stdout().flush();
        let mut user_input: String = String::new();
        std::io::stdin().read_line(&mut user_input);
        user_input = String::from(user_input.trim());
        if user_input.len() != 5 {
            print!("You need 5 letters");
            continue;
        }
        times = times + 1;

        let color_user: Vec<Color> = add_user_colors(user_input); 

        let result = check_equality(&guess, &color_user);

        fancy_print_guess(&color_user);
        if result == true {
            println!("YOU WIN! With {} try", times);
            play = false;
        } else {
            println!("You have {} colors well placed !", number_of_well_placed_pawns(&guess, &color_user));
            println!("And have {} colors not well placed !", number_of_not_well_placed_pawns(&guess, &color_user));

        }
    }
}


fn add_user_colors(user_input: String) -> Vec<Color> {
    
    let mut color_user: Vec<Color> = Vec::new(); 
    for c in user_input.chars() {
        match c {
            'B' => color_user.push(Color::Blue),
            'P' => color_user.push(Color::Pink),
            'O' => color_user.push(Color::Orange),
            'Y' => color_user.push(Color::Yellow),
            'G' => color_user.push(Color::Green),
            'W' => color_user.push(Color::White),
            'R' => color_user.push(Color::Red),
            'p' => color_user.push(Color::Purple),
            _ => color_user.push(Color::None)
        }
    }
    return color_user;
}

fn get_random_colors() -> Vec<Color> {
    let mut rng = rand::thread_rng();
    let mut colors: Vec<Color> = Vec::new();
    for _value in 0..5 {
        let random_number = rng.gen_range(0..7);
        match random_number {
            0 => colors.push(Color::Blue),
            1 => colors.push(Color::Pink),
            2 => colors.push(Color::Orange),
            3 => colors.push(Color::Yellow),
            4 => colors.push(Color::Green),
            5 => colors.push(Color::White),
            6 => colors.push(Color::Red),
            7 => colors.push(Color::Purple),  
           _ => {}
        }
    }

    return colors;
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut well_placed = 0;
    for (color,_y) in guess.iter().enumerate() {
        if guess[color] == secret[color] {
            well_placed = well_placed + 1;
        }
    }
    return well_placed; 
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut not_well_placed = 0;
    let mut well_placed_table = [0;5];

    for (i,_x) in guess.iter().enumerate() {
        if guess[i] == secret[i] {
            well_placed_table[i] = 1;
        }
    }
    
    for (si,sx) in secret.iter().enumerate() {
        if well_placed_table[si] == 0 {
            for (gi,gx) in guess.iter().enumerate() {
                if well_placed_table[gi] == 0 && si != gi && sx == gx {
                    not_well_placed = not_well_placed + 1;
                }
            }
        }

    }
    return not_well_placed;
}


fn check_equality(guess: &[Color], user_input: &[Color]) -> bool {
    for (i,_x) in guess.iter().enumerate() {
        if guess[i] != user_input[i]{
            return false;
        }
    
    }
    return true;
}

fn fancy_print_guess(guess: &[Color]){
    for color in guess{
        match color {
            Color::Blue => print!("{}", Colour::White.paint("B")), 
            Color::Pink => print!("{}", Colour::White.paint("Pi")), 
            Color::Orange => print!("{}", Colour::White.paint("O")), 
            Color::Yellow => print!("{}", Colour::White.paint("Y")), 
            Color::Green => print!("{}", Colour::White.paint("G")), 
            Color::White => print!("{}", Colour::White.paint("W")), 
            Color::Red => print!("{}", Colour::White.paint("R")), 
            Color::Purple => print!("{}", Colour::White.paint("Pu")), 
            Color::None => print!("ERROR"),
        }
    }
    println!("");
}
