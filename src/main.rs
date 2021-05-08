
use std::io::{self, BufRead};
use std::io::Write;
use std::io::stdout;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum Colors {
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
    Orange,
    Lime,
    Charcoal
}

fn fancy_print_guess(vec: Vec<Colors>) {
    for color in vec {
        if matches!(color, Colors::Red) { print!("R")}
        if matches!(color, Colors::Blue) { print!("B")}
        if matches!(color, Colors::Green) { print!("G")}
        if matches!(color, Colors::Yellow) { print!("Y")}
        if matches!(color, Colors::Purple) { print!("P")}
        if matches!(color, Colors::Orange) { print!("O")}
        if matches!(color, Colors::Lime) { print!("L")}
        if matches!(color, Colors::Charcoal) { print!("C")}
    }
    println!()
}

fn main()
{
    let mut guess: Vec<Colors> = Vec::with_capacity(5);
    guess.push(Colors::Blue);
    guess.push(Colors::Charcoal);
    guess.push(Colors::Lime);
    guess.push(Colors::Orange);
    guess.push(Colors::Yellow);

    println! {"{:?}", guess}
    println!("Hello, world! Welcome to the Super mastermind ! (Type Exit to quit the program)");
    fancy_print_guess(guess);

    let stdin = io::stdin();

    loop {
        print!("> ");
        stdout().flush();
        let line1 = stdin.lock().lines().next().unwrap().unwrap();
        println!("You writed: {}", line1);
        if line1.eq(&String::from("Exit")) {
            break;
        }
    }
}
