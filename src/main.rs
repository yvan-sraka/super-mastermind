use ansi_term::Colour::{Blue, Green, Cyan, Purple, Red, Yellow, White, RGB};
use std::io::{stdin};

#[derive(Debug)]
#[derive(PartialEq, Eq)]
enum Color {
    Blue,
    Cyan,
    Green,
    Orange,
    Purple,
    Red,
    Yellow,
    White,
}

fn fancy_print_guess(guess: &[Color]) {
    for color in guess {
        match color {
            Color::Blue => print!("{}", Blue.paint("B")),
            Color::Green => print!("{}",Green.paint("G")),
            Color::Cyan => print!("{}", Cyan.paint("C")),
            Color::Orange => print!("{}", RGB(255, 96, 0).paint("O")),
            Color::Purple => print!("{}", Purple.paint("P")),
            Color::Red => print!("{}", Red.paint("R")),
            Color::Yellow => print!("{}", Yellow.paint("Y")),
            Color::White => print!("{}", White.paint("W")),
        } 
    }
    println!();
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut count: i32 = 0;
    for i in secret.iter().zip(guess.iter()) {
        let (item1, item2) = i;
        if item1 == item2 {count += 1;}
    }
    count
}

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    let mut count: i32 = 0;
    for i in secret.iter().zip(guess.iter()) {
        let (item1, item2) = i;
        if item1 != item2 {count += 1;}
    }
    count
}

fn main() {
    let secret: Vec<Color> = vec![Color::Red, Color::Red, Color::Blue, Color::Yellow, Color::Green];

    'outer: loop {
        let mut guess = vec![];
        let mut s=String::new();
        stdin().read_line(&mut s);
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        if s.len() != 5 {
            println!("The input must be 5 charaters long");
            break;
        }

        s.to_uppercase();

        for item in s.chars() {
            match item {
                'B' => guess.push(Color::Blue),
                'C' => guess.push(Color::Cyan),
                'G' => guess.push(Color::Green), 
                'O' => guess.push(Color::Orange), 
                'P' => guess.push(Color::Purple),
                'R' => guess.push(Color::Red),
                'Y' => guess.push(Color::Yellow), 
                'W' => guess.push(Color::White),
                _ =>  {
                    println!("Please use:\n'B' for Blue\n'C' for Cyan\n'G' for Green\n'O' for Orange\n'P' for Purple\n'R' for Red\n'Y' for Yellow\n'W' for White");
                    continue 'outer;},
            }
        }
        fancy_print_guess(&guess);
        let wellPlacedPawns = number_of_well_placed_pawns(&secret, &guess);
        let notWellPlacedPawns = number_of_not_well_placed_pawns(&secret, &guess);
        println!("Number of well placed pawns : {}", wellPlacedPawns);
        println!("Number of not well placed pawns : {}", notWellPlacedPawns);

        if wellPlacedPawns != 5 { continue 'outer; }

        println!("Well great mind you found it");

        println!("Wanna play again ? Y/N : ");
        'keep: loop {
            s = String::from("");
            stdin().read_line(&mut s);

            if let Some('\n')=s.chars().next_back() {
                s.pop();
            }
            if let Some('\r')=s.chars().next_back() {
                s.pop();
            }

            match &s[..] {
                "Y" => continue 'outer,
                "N" => break 'outer,
                _ => println!("please put 'Y' for Yes or 'N' for No"),
            }
        }
    } 
}