mod enum_color;
use enum_color::Colors;

use std::io;
use std::io::prelude::*;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

type ColorParsingError = String;

trait FromLetter {
    fn from_letter(l: char) -> Result<Colors, ColorParsingError>;
}

impl FromLetter for Colors {
    fn from_letter(l: char) -> Result<Colors, ColorParsingError> {
        match l {
            'b' | 'B' => Ok(Colors::Blue),
            'r' | 'R' => Ok(Colors::Red),
            'p' | 'P' => Ok(Colors::Purple),
            'g' | 'G' => Ok(Colors::Green),
            'y' | 'Y' => Ok(Colors::Yellow),
            'i' | 'I' => Ok(Colors::Pink),
            'c' | 'C' => Ok(Colors::Cyan),
            'm' | 'M' => Ok(Colors::Magenta),
            _ => Err(ColorParsingError::from(format!(
                "{} is not available as a color letter !",
                l
            ))),
        }
    }
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

fn pause_terminal() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any key to continue.").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn fancy_print_color(color: &Colors) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let write_color: Color = match color {
        Colors::Blue => Color::Blue,
        Colors::Cyan => Color::Cyan,
        Colors::Green => Color::Green,
        Colors::Magenta => Color::Magenta,
        Colors::Pink => Color::Rgb(255, 111, 255),
        Colors::Purple => Color::Rgb(153, 0, 255),
        Colors::Red => Color::Red,
        Colors::Yellow => Color::Yellow,
    };
    if let Err(e) = stdout.set_color(ColorSpec::new().set_fg(Some(write_color))) {
        println!("Error occured writing: {}", e.to_string());
    } else if let Err(e) = write!(&mut stdout, " ⦿ ") {
        println!("Error occured writing: {}", e.to_string());
    }

    if let Err(e) = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))) {
        println!("Error occured writing: {}", e.to_string());
    } else if let Err(e) = writeln!(&mut stdout, "") {
        println!("Error occured writing: {}", e.to_string());
    }
}

fn prompt_stop(stop_flag: &mut bool) {
    let mut buffer = String::new();

    println!("Exit program now?(y/N)");

    io::stdin()
        .read_line(&mut buffer)
        .ok()
        .expect("An error ocured reading input.");

    buffer = String::from(buffer.strip_suffix("\n").unwrap_or_default());

    if buffer.eq_ignore_ascii_case("yes") || buffer.eq_ignore_ascii_case("y") {
        *stop_flag = true;
        println!("stop flag should be set to true");
    }
}

type InputErr = String;

fn parse_input(input: &String) -> Result<Vec<Colors>, InputErr> {
    if input.is_empty() {
        return Err(InputErr::from("You provided an empty string..."));
    } else if input.len() != 5 {
        return Err(InputErr::from(format!(
            "You must enter 5 caracters. Got {}",
            input.len()
        )));
    }

    let mut res: Vec<Colors> = Vec::new();

    for letter in input.chars() {
        let opt_color = Colors::from_letter(letter);
        if let Ok(color) = opt_color {
            res.push(color);
        } else {
            return Err(opt_color.unwrap_err());
        }
    }

    return Ok(res);
}

fn main() {
    // let guess = vec![
    //     Colors::Blue,
    //     Colors::Green,
    //     Colors::Red,
    //     Colors::Red,
    //     Colors::Magenta,
    // ];
    // let guess_2 = vec![
    //     Colors::Purple,
    //     Colors::Yellow,
    //     Colors::Pink,
    //     Colors::Cyan,
    //     Colors::Magenta,
    // ];

    let mut stop_flag = true;

    while !stop_flag {
        loop {
            clear_terminal();
            println!("Please input a number.");
            let stdin = io::stdin();

            let mut buffer = String::new();

            stdin
                .read_line(&mut buffer)
                .ok()
                .expect("An error ocured reading input.");

            buffer.pop();

            let input = parse_input(&buffer);

            if let Ok(array) = input {
                println!("{:?}", array);
                break;
            } else {
                println!("{}", input.err().unwrap())
            }

            pause_terminal();
        }
        prompt_stop(&mut stop_flag);
    }

    fancy_print_color(&Colors::Blue);
}
