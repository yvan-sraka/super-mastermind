use ansi_term::Colour;
use std::io;
use std::io::Write;
use rand::Rng;
use std::fmt;

#[derive(Debug, PartialEq)]
enum Color{
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
    White,
    Cyan,
    Maroon
}


fn main() {
    let SECRET_LENGTH: i32 = 6;
    // let mut vec: Vec<Color> = Vec::new();
    // guess.push(Color::Red);
    // guess.push(Color::Blue);
    // guess.push(Color::Green);
    // guess.push(Color::Yellow);
    // guess.push(Color::Purple);
    // guess.push(Color::White);
    // guess.push(Color::Cyan);
    // guess.push(Color::Maroon);

    //rintln!("{:?}", Color::Maroon);

    // print_vec(guess);

    //println!("{}", Colour::RGB(102, 51, 0).paint("M"));
    let secret = generate_combi(SECRET_LENGTH);
    let mut tries = 0;
    let mut invalid_guess = false;
    print!("Valid colors : ");
    fancy_print_guess(&[Color::Red, Color::Blue, Color::Yellow, Color::Green, Color::Maroon, Color::Purple, Color::White, Color::Cyan]);


    loop{
        let mut guess = vec![];
        let mut input = String::new();
        print!("Enter your guess: ");
        io::stdout().flush();
        io::stdin().read_line(&mut input);
        println!("Input : {}", input);
        let letters = input.trim();
        for letter in letters.chars(){
            let color = match letter {
                'R' => Color::Red,
                'B' => Color::Blue,
                'G' => Color::Green,
                'Y' => Color::Yellow,
                'P' => Color::Purple,
                'W' => Color::White,
                'C' => Color::Cyan,
                'M' => Color::Maroon,
                _ => {
                    invalid_guess = true; 
                    println!("Color {} is invalid", letter);
                    break;
                }
            };
            guess.push(color);
        }

        if secret.len() != guess.len() {
            invalid_guess = true;
            println!("Secret combination has {} words. Please type the same amount of characters", SECRET_LENGTH)
        }
        if secret == vec_to_string(guess) {
            println!("Congrats ! You guessed the secret combination !");
            break;
        }
        if invalid_guess{
            invalid_guess = false;
        } else {
            tries += 1;
        }
        println!("{}", tries);

        // let secretVec = string_to_vec(secret);

        // let good = number_of_well_placed_pawns(&secretVec, &guess);
        // let wrong = number_of_not_well_placed_pawns(&secretVec, &guess);

        // println!("{} colors ok, {} colors wrong", good, wrong);
        // fancy_print_guess(&guess);
        // println!();
    }
}


fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    secret
    .iter()
    .enumerate()
    .filter(|&(i, color)| guess[i] == *color)
    .count() as i32
}

fn number_of_not_well_placed_pawns(secret: &Vec<Color>, guess: &Vec<Color>) -> i32 {
    (secret.len() as i32) - number_of_well_placed_pawns(secret, guess)
}

fn string_to_vec(secret: String) -> Vec<Color>{
    let mut vec: Vec<Color> = Vec::new();
    for letter in secret.chars() {
        match letter {
            'R' => vec.push(Color::Red),
            'B' => vec.push(Color::Blue),
            'G' => vec.push(Color::Green),
            'Y' => vec.push(Color::Yellow),
            'P' => vec.push(Color::Purple),
            'W' => vec.push(Color::White),
            'C' => vec.push(Color::Cyan),
            'M' => vec.push(Color::Maroon),
            _ => print!("rien")
        }
    }
    vec
}

fn vec_to_string(guess: Vec<Color>) -> String{
    let mut s: String = String::from("");
    for color in guess {
        match color{
            Color::Red => s.push('R'),
            Color::Blue => s.push('B'),
            Color::Green => s.push('G'),
            Color::Yellow => s.push('Y'),
            Color::Purple => s.push('P'),
            Color::White => s.push('W'),
            Color::Cyan => s.push('C'),
            Color::Maroon => s.push('M'),
        }
    }
    return s;
}

fn print_vec(guess: Vec<Color>){
    println!("Guess :");
    for color in guess {
        println!("{:?}", color);
    }
}

fn fancy_print_guess(guess: &[Color]){
    let mut vec = Vec::new();
    for color in guess {
        match color{
            Color::Red => vec.push(Colour::Red.paint("R")),
            Color::Blue => vec.push(Colour::Blue.paint("B")),
            Color::Green => vec.push(Colour::Green.paint("G")),
            Color::Yellow => vec.push(Colour::Yellow.paint("Y")),
            Color::Purple => vec.push(Colour::Purple.paint("P")),
            Color::White => vec.push(Colour::White.paint("W")),
            Color::Cyan => vec.push(Colour::Cyan.paint("C")),
            Color::Maroon => vec.push(Colour::Fixed(89).paint("M"))
        }
    }
    for el in vec {
        print!("{}", el);
    }
    println!();
}

fn generate_combi(range: i32) -> String{
    const CHARSET: &str = "RBGYPWCM";
    let vec: Vec<char> = CHARSET.chars().collect();
    let mut rng = rand::thread_rng();
    let secret: String = (0..range)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            vec[idx] as char
        })
        .collect();
    println!("Secret : {}", secret);
    return secret;
}
