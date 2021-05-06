use ansi_term::Colour;
use std::io;
use rand::prelude::*;

const LETTERS: [char; 8] = ['R', 'G', 'B', 'Y', 'C', 'P', 'O', 'W'];

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Color{
    Red(),
    Green(),
    Blue(),
    Yellow(),
    Cyan(),
    Purple(),
    Orange(),
    White(),
    Empty()
}

// //get a Color and return a char with color
// fn fancy_color_letter(color: &Color) -> String{
//     match color {
//         Color::Red() => Colour::RGB(255, 0, 0).paint("R").to_string(),
//         Color::Green() => Colour::RGB(0, 255, 0).paint("G").to_string(),
//         Color::Blue() => Colour::RGB(0, 0, 255).paint("B").to_string(),
//         Color::Yellow() => Colour::RGB(255, 255, 0).paint("Y").to_string(),
//         Color::Cyan() => Colour::RGB(0, 255, 255).paint("C").to_string(),
//         Color::Purple() => Colour::RGB(255, 0, 255).paint("P").to_string(),
//         Color::Orange() => Colour::RGB(255, 127, 0).paint("O").to_string(),
//         Color::White() => Colour::RGB(255, 255, 255).paint("W").to_string()
//     }
// }

fn fancy_color_letter(color: &Color) -> String{
    match color {
        Color::Red() => Colour::RGB(255, 0, 0).bold().paint("R").to_string(),
        Color::Green() => Colour::RGB(0, 255, 0).bold().paint("G").to_string(),
        Color::Blue() => Colour::RGB(0, 0, 255).bold().paint("B").to_string(),
        Color::Yellow() => Colour::RGB(255, 255, 0).bold().paint("Y").to_string(),
        Color::Cyan() => Colour::RGB(0, 255, 255).bold().paint("C").to_string(),
        Color::Purple() => Colour::RGB(255, 0, 255).bold().paint("P").to_string(),
        Color::Orange() => Colour::RGB(255, 127, 0).bold().paint("O").to_string(),
        Color::White() => Colour::RGB(255, 255, 255).bold().paint("W").to_string(),
        Color::Empty() => String::from("Empty")
    }
}


//print string
fn fancy_print_guess(guess: &[Color]){
    for x in guess{
        print!("{}", fancy_color_letter(x));
    }
    print!("\n");
}

//take a letter and return the corresponding Color from the enum
fn color_enum_from_letter(letter: char) -> Color{
    match letter {
        'R' => Color::Red(),
        'G' => Color::Green(),
        'B' => Color::Blue(),
        'Y' => Color::Yellow(),
        'C' => Color::Cyan(),
        'P' => Color::Purple(),
        'O' => Color::Orange(),
        'W' => Color::White(),
        _ => Color::Empty()
    }
}

//compare guess and answer, return true if the vec are the same
fn compare_vec(answer: &Vec<Color>, guess: &Vec<Color>) -> bool{
    if answer.len() != guess.len() {
        return false;
    } else {
        for i in 0..5 {
            if answer[i] != guess[i]{
                return false;
            }
        }
    }
    return true;
}

//generate a Vec with random color
fn answer_generator() -> Vec<Color>{
    let mut answer: Vec<Color> = Vec::new();

    while answer.len() <= 4 {
        let mut rng = rand::thread_rng();
        let rand_number = rng.gen_range(0..8);

        answer.push(color_enum_from_letter(LETTERS[rand_number]))

    }
    return answer;
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut counter: i32 = 0;

    for i in 0 .. guess.len(){
        if guess[i] == secret[i]{
            counter = counter + 1;
        }
    }

    return counter;
}


fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32{
    let mut counter: i32 = 0;

    let mut guess_pos = vec![0,0,0,0,0];
    let mut secret_pos = vec![0,0,0,0,0];

    for i in 0.. guess.len() {
        if secret[i] == guess[i]{
            guess_pos.remove(i);
            guess_pos.insert(i, 1);
            secret_pos.remove(i);
            secret_pos.insert(i, 1);
        }
    }

    for i in 0.. guess.len() {
        for j in 0.. guess.len() {
            if guess_pos[i] == 1 || secret_pos[j] == 1{
                continue;
            } else {
                if guess[i] == secret[j]{
                    counter = counter + 1;
                    guess_pos.remove(i);
                    guess_pos.insert(i, 1);
                    secret_pos.remove(j);
                    secret_pos.insert(j, 1);
                }
            }
        }
    }

    return counter;
}

fn print_rules() {
    println!("{} for {}", Colour::RGB(255, 0, 0).bold().paint("R"), Colour::RGB(255, 0, 0).bold().paint("Red"));
    println!("{} for {}", Colour::RGB(0, 255, 0).bold().paint("G"), Colour::RGB(0, 255, 0).bold().paint("Green"));
    println!("{} for {}", Colour::RGB(0, 0, 255).bold().paint("B"), Colour::RGB(0, 0, 255).bold().paint("Blue"));
    println!("{} for {}", Colour::RGB(255, 255, 0).bold().paint("Y"), Colour::RGB(255, 255, 0).bold().paint("Yellow"));
    println!("{} for {}", Colour::RGB(0, 255, 255).bold().paint("C"), Colour::RGB(0, 255, 255).bold().paint("Cyan"));
    println!("{} for {}", Colour::RGB(255, 0, 255).bold().paint("P"), Colour::RGB(255, 0, 255).bold().paint("Purple"));
    println!("{} for {}", Colour::RGB(255, 127, 0).bold().paint("O"), Colour::RGB(255, 127, 0).bold().paint("Orange"));
    println!("{} for {}", Colour::RGB(255, 255, 255).bold().paint("W"), Colour::RGB(255, 255, 255).bold().paint("White"));
}

fn main(){
    //creation of answer
    let answer: Vec<Color> = answer_generator();

    fancy_print_guess(&answer);

    let mut count: u8 = 0;

    println!("Type 5 letters in this range : {:?}", print_rules());
    loop {
        //vector that will be use to build the guess from the player input
        let mut guess: Vec<Color> = Vec::new();
        //if the player input is incorrect, is_input_correct will be false 
        let mut is_input_correct = true;

        println!("It's the round : {}", count);

        //create string
        let mut input = String::new();
        //put the stdin inside input string
        match io::stdin().read_line(&mut input) {
            Ok(_) => continue,
            Err(e) => {
                println!("{}", e);
            }
        }

        //pop the last cha that is a ghost char that can't be work with
        input.pop();

        //check that the input is 5 char long
        if input.chars().count() != 5 {
            is_input_correct = false;
        }

        //iterate the input in uppercase
        for letter in input.to_uppercase().chars() {
            //break if a char is not contains in the LETTER vec constant
            if !LETTERS.contains(&letter){
                is_input_correct = false;
                break;
            }
        }

        //pass if the input is correct
        if is_input_correct {
            //round + 1
            count = count + 1;
            //iterate the input in uppercase
            for letter in input.to_uppercase().chars() {
                //push each char inside the guess: Vec<Color> 
                guess.push(color_enum_from_letter(letter));
            }
            
            fancy_print_guess(&guess);

            if compare_vec(&answer, &guess){
                println!("You guessed the answer! Well done.");
                println!("You tried {} time(s).", count);
                break;
            } else {
                println!("Wrong try again.");

                println!("There is {} well placed pawns.", number_of_well_placed_pawns(&answer, &guess));
                println!("There is {} not well placed pawns.", number_of_not_well_placed_pawns(&answer, &guess));
                println!();
            }

        } else {
            println!("Type 5 letters in this range : {:?}", print_rules());
            println!("Wrong input.");
            println!("The count has not been increased.");
            println!();
        }

    }

}
