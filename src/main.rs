use std::io;
use ansi_term::Colour::{Red,
    Green,
    Yellow,
    Blue,
    Cyan,
    Black,
    Purple,};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Color {
        Red,
        Green,
        Yellow,
        Blue,
        Cyan,        
        Purple,
    }

fn fancy_print_guess(guess: &mut Vec<Color>) {
    //let mut fancy_print= Vec::new();
    println!("the colors to guess");
    let size = guess.len();     
    for n in 0.. size{
       if guess[n] == Color::Red
        {
        print!(" {}", Red.paint("R"));
         }
       else  if guess[n] == Color::Green
        {
        print!(" {}", Green.paint("G"));
       }
       else  if guess[n] == Color::Yellow 
       {
        print!(" {}", Yellow.paint("Y"));
        }
       else  if guess[n] == Color::Blue
        {
        print!(" {}", Blue.paint("B"));
        }
       else  if guess[n] == Color::Cyan 
       {
        print!(" {}", Cyan.paint("C"));
       }
 }
     println!("");
     //println!("{:?}",fancy_print);
}

fn number_of_well_placed_pawns(_input: &mut str,_fancy_print: &mut str) -> i32{

    let mut number =0 ;
    for (c1, c2) in _input.chars().zip(_fancy_print.chars()) {
        if c1 == c2 {
            number = number+1;
        }
    }

    return number;
}
fn number_of_not_well_placed_pawns(_input: &mut str,_fancy_print: &mut str) -> i32{

    let mut number =0 ;
    for (c1, c2) in _input.chars().zip(_fancy_print.chars()) {
        if c1 != c2 {
            number = number+1;
        }
    }
    return number;
}

fn infinite_loop_compare(guess: &mut Vec<Color>){

    let mut count =0;
    let size = guess.len();
    let mut fancy_print= String::new();
    for n in 0..size{
        if guess[n] == Color::Red
        {
         fancy_print.push('R');
         }
       else  if guess[n] == Color::Green
        {
            fancy_print.push('G');
        }
       else  if guess[n] == Color::Yellow 
       {
        fancy_print.push('Y');
    }
       else  if guess[n] == Color::Blue
        {
            fancy_print.push('B');
        }
       else  if guess[n] == Color::Cyan 
       {
        fancy_print.push('C');
    }
    }
    //println!("{}",fancy_print);
    println!("Please Enter the 5 colors to guess");
    loop{
        let mut input = String::new();
        match io::stdin().read_line(&mut input){
            Ok(_) => {
                let size_vector = input.len()-1;
                if size_vector != 5 {
                    println!("{}",Red.paint("vecotr must have five character"));
                    println!("{}",Yellow.paint("Please Enter Again 5 character "));
                    continue;
                }
                else if input.trim() != fancy_print{
                    
                    println!("{}",Red.paint("your enter is not good !!!"));
                    count = count + 1;
                    println!("this guess false number {}",count);
                    //function  help user to know number of well placed pawns
                    println!("number of well placed pawns : {}",number_of_well_placed_pawns(&mut input,&mut fancy_print));
                    //function help user to know number of not well placed pawns
                    println!("number of not well placed pawns : {}",number_of_not_well_placed_pawns(&mut input,&mut fancy_print));
                    println!("{}",Yellow.paint("Please Enter Again 5 character "));
                    continue;
                }
                else{

                    println!("{} : {} ",Green.paint("Congratulations Your guess is good "),input);
                    break;

                }
            },
            Err(e) => { 
                println!("Something went wrong: {}", e);
                break;
        }
    }
  }
}
fn main() {

    //nom de vecteur guess
    let mut guess: Vec<Color> = Vec::new();
    //combination of 5 colors.
    guess.push(Color::Red);
    guess.push(Color::Blue);
    guess.push(Color::Green);
    guess.push(Color::Yellow);
    guess.push(Color::Green);
    //affichage des caractères non colorés 
    println!("{:?}",guess);
    //affichage des caractères colorés 
    fancy_print_guess(&mut guess);
    //Make an infinite loop which at each turn: read a string and print it!
    infinite_loop_compare(&mut guess);

    
}


