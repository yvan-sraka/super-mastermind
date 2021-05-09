use ansi_term::Colour::Red;
use std::{fmt, io};
use ansi_term::Colour::Green;
use rand;
use rand::{Rng};
use rand::distributions::{ Distribution, Standard};

#[derive(Debug)]
#[derive(PartialEq)]

enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Indigo,
    Purple,
    Black,
    White,
    NoColor
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0..=7) {
            0=>Color::Red,
            1=>Color::Orange ,
            2=>Color::Yellow,
            3=>Color::Black,
            4=>Color::White,
            5=>Color::Green,
            6=>Color::Purple,
            _=>Color::Indigo
        }
    }
}

fn generate_secret()->Vec<Color>{
    let v :Vec<Color>=   (0..4).map(|_| rand::random()).collect();
    return v;
}
fn main() {
    println!("Guess the color!");

    let secret_color=generate_secret();

    // println!("{:?}", fancy_print_guess(&secret_color));
    let mut number_of_guess =0;
    loop{
        println!("Please input your guess. round {}",number_of_guess);
        let foo = String::new();
        let mut guess = foo;
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let colors =str_to_vec_colors(guess);
        if !invalid_len(&secret_color,&*colors) {
            number_of_guess+=1;
            println!("{:?}", fancy_print_guess(&colors));
            println!("{}",
                     Green.paint(
                         format!("{} {}","Number of well placed",
                                 number_of_well_placed_pawns(&secret_color,
                                                             &colors))).to_string()
            );
            println!("{}",Red.paint(format!("{} {}","Number of not well placed",
                                            number_of_not_well_placed_pawns(&secret_color,&colors))).to_string()
            );
            if found_suit(colors, &secret_color){
                println!("{}",Green.paint("You won!"));
                break;
            }

        }else{
            println!("{}",Red.paint(format!("{}","Invalid len")).to_string())
        }
    }
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32{
    return guess.len() as i32-number_of_well_placed_pawns(secret,guess);
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32{
    let mut number_of_well_placed =0;
    for (i,c) in secret.iter().enumerate() {
        if *c==guess[i] {
            number_of_well_placed+=1;
        }
    }
    return number_of_well_placed;
}

fn str_to_vec_colors(guess:String) ->Vec<Color>{
    println!("{}",guess);
    let mut colors:Vec<Color>=Vec::new();
    for  c in guess.trim().chars() {
        colors.push(match c {
            'R'=>Color::Red,
            'O'=>Color::Orange ,
            'Y'=>Color::Yellow,
            'B'=>Color::Black,
            'W'=>Color::White,
            'G'=>Color::Green,
            'P'=>Color::Purple,
            'I'=>Color::Indigo,
            _ => {
                Color::NoColor;
                println!("{}",Red.paint(format!("{} {}",c,"not exist in enum Color")).to_string());
                break;
            }
        });
    }
    colors
}

fn found_suit( colors:Vec<Color>,secret:&Vec<Color>)->bool{
    let mut good =0;
    for (index,color) in colors.iter().enumerate() {
        if *color==secret[index] {
            good+=1;
        }else{
            return false
        }
    }
    return if good == secret.len() {
        true
    } else {
        false
    }
}

fn invalid_len(secret:&[Color],guess:&[Color]) ->bool{
    if  guess.len()!=secret.len(){
        return true
    }

    return false
}

fn fancy_print_guess(guess:&[Color]) -> String {
    let mut str:String="".to_owned();
    for n in guess {
        str.push_str(match n {
            Color::Red => "R",
            Color::Orange =>"O",
            Color::Yellow=>"Y",
            Color::Black=>"B",
            Color::White=>"W",
            Color::Green=>"G",
            Color::Purple=>"P",
            Color::Indigo=>"I",
            _ => {
                println!("{}",Red.paint(format!("{} {}",n,"not exist in enum Color")).to_string());
                ""
            }
        });
    }
    return str;
}