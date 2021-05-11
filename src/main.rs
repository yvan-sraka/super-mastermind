#[derive(Debug,PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Cyan,
    White,
    Black,
}
use rand::prelude::*;
use ansi_term::Colour::{Red,Green,Blue,Yellow,Purple,Cyan,White,Black};
use std::io::{self, Read};

fn is_same_color(color1 : &Color, color2 : &Color)->bool{
    
    return color1 == color2
}

fn fancy_print_guess(guess: &[Color]) -> (){
    for color in guess.iter(){
        match color {
            &Color::Red     => print!("{}",Red.paint("R")),
            &Color::Green   => print!("{}",Green.paint("G")),
            &Color::Blue    => print!("{}",Blue.paint("B")),
            &Color::Yellow  => print!("{}",Yellow.paint("Y")),
            &Color::Purple  => print!("{}",Purple.paint("P")),
            &Color::Cyan    => print!("{}",Cyan.paint("C")),
            &Color::White   => print!("{}",White.paint("W")),
            _               => (),
        }
    }
    println!("");
}

fn color_verification(myGuess: &[Color]) -> bool{
    for i in 0..5{
        if myGuess[i] == Color::Black { 
            return false;
        }
    }
    return true;
}

fn string_verification(myCharsVec: &[char]) -> bool {
    if myCharsVec.len()-1 < 5 {
        return false;
    }
    for color in myCharsVec.iter().take(myCharsVec.len()-1){
        if !color.is_alphabetic() {
            return false;
        }
    }
    return true;
}

fn read_user_input() -> (){
    let mut input = String::new();
    let mut round = 1;
    let mut secret = Vec::<Color>::new();
    generate_secret(&mut secret);
    loop{
        println!("round {}",round);
        match io::stdin().read_line(&mut input) {
        Ok(n) => {
            let vec: Vec<char> = input.chars().collect();
            let mut input_valid = true;
            
            if string_verification(&vec) {
                
                input = input.to_uppercase();
                let vec: Vec<char> = input.chars().collect();
                //fancy_print_guess(vec);
                
                let mut colorVec = Vec::<Color>::new(); 
                colorVec = stringVecToColorVec(&vec,colorVec);
                    round +=1; 
                    println!("You input : ");
                    fancy_print_guess(&colorVec);
                    if (number_of_well_placed_pawns(&secret,&colorVec) == 5 && number_of_not_well_placed_pawns(&secret,&colorVec) == 0){
                        println!("You got it !!");
                        break;
                    }else{
                         println!("Nombre de couleurs bien placé : {}",number_of_well_placed_pawns(&secret,&colorVec));
                         println!("Nombre de couleurs mal placé  : {}",number_of_not_well_placed_pawns(&secret,&colorVec));
                    }
            
            }else{
                input_valid = false;
            }
            if !input_valid{
                println!("Your input must be at least 5 char from \nR : Red,
                \nG : Green
                \nB : Blue
                \nY : Yellow
                \nP : Purple
                \nC : Cyan
                \nW : White");
            }

            input.clear();

        }
        Err(error) => println!("error: {}", error),
        }
    }
}
fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32{
    let mut res : i32 = 0;
    let mut max = 0;
    if secret.len()>=guess.len(){
        max = guess.len();
        let diff : i32 = (secret.len() as i32) - (guess.len() as i32);
        res += diff;
    } else{
        let diff : i32 =  (guess.len() as i32) - (secret.len() as i32);
        res += diff;
        max = secret.len();
    } 
    for i in 0..max{
        let mut colorguessed = &guess[i];
        if !is_same_color(&secret[i],colorguessed){
            res += 1;
        }
    }
    return res;
}

fn generate_secret(secret : &mut Vec<Color>){
    let mut rng = rand::thread_rng();
    
    let mut nums: Vec<i32> = (0..7).collect();
    nums.shuffle(&mut rng);
    for num in nums.iter().take(5){
        match num{
            0 => secret.push(Color::Red),
            1 => secret.push(Color::Green),
            2 => secret.push(Color::Blue),
            3 => secret.push(Color::Yellow),
            4 => secret.push(Color::Purple), 
            5 => secret.push(Color::Cyan),
            6 => secret.push(Color::White), 
            _ => secret.push(Color::White),
        }
    }
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32{
    let mut res = 0;
    let mut max = 0;
    if secret.len()>=guess.len(){
        max = guess.len();
    } else{
        max = secret.len();
    } 
    for i in 0..max{
        let colorguessed = &guess[i];
        if is_same_color(&secret[i],colorguessed){
            res += 1;
        }
    }
    return res;
}

fn stringVecToColorVec(stringVec : &[char], mut colorVec : Vec<Color>) ->  Vec<Color> {
    for charInVec in stringVec.iter(){
        match charInVec {
            'R' => colorVec.push(Color::Red),
            'G' => colorVec.push(Color::Green),
            'B' => colorVec.push(Color::Blue),
            'Y' => colorVec.push(Color::Yellow),
            'P' => colorVec.push(Color::Purple), 
            'C' => colorVec.push(Color::Cyan),
            'W' => colorVec.push(Color::White), 
            _   => colorVec.push(Color::Black),
        }
    }
    return colorVec;
}

fn main() {
    //let guess: Vec<Color> = vec![Color::Red,Color::Green,Color::Blue,Color::Yellow,Color::Purple];
//    println!("Hello, world!");
    //fancy_print_guess(&guess);
    println!("Infinite loop");
    read_user_input();
    //println!("`{:?}`", guess);
}
