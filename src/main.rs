mod enum_color;
use enum_color::Colors;

use std::io;
use std::io::prelude::*;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};


type ColorParsingError = String;

const BANNER: &str = 
" ▄▄▄▄▄▄▄ ▄▄   ▄▄ ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄▄▄▄      ▄▄   ▄▄ ▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄▄▄▄   ▄▄   ▄▄ ▄▄▄ ▄▄    ▄ ▄▄▄▄▄▄  
█       █  █ █  █       █       █   ▄  █    █  █▄█  █      █       █       █       █   ▄  █ █  █▄█  █   █  █  █ █      █ 
█  ▄▄▄▄▄█  █ █  █    ▄  █    ▄▄▄█  █ █ █    █       █  ▄   █  ▄▄▄▄▄█▄     ▄█    ▄▄▄█  █ █ █ █       █   █   █▄█ █  ▄    █
█ █▄▄▄▄▄█  █▄█  █   █▄█ █   █▄▄▄█   █▄▄█▄   █       █ █▄█  █ █▄▄▄▄▄  █   █ █   █▄▄▄█   █▄▄█▄█       █   █       █ █ █   █
█▄▄▄▄▄  █       █    ▄▄▄█    ▄▄▄█    ▄▄  █  █       █      █▄▄▄▄▄  █ █   █ █    ▄▄▄█    ▄▄  █       █   █  ▄    █ █▄█   █
 ▄▄▄▄▄█ █       █   █   █   █▄▄▄█   █  █ █  █ ██▄██ █  ▄   █▄▄▄▄▄█ █ █   █ █   █▄▄▄█   █  █ █ ██▄██ █   █ █ █   █       █
█▄▄▄▄▄▄▄█▄▄▄▄▄▄▄█▄▄▄█   █▄▄▄▄▄▄▄█▄▄▄█  █▄█  █▄█   █▄█▄█ █▄▄█▄▄▄▄▄▄▄█ █▄▄▄█ █▄▄▄▄▄▄▄█▄▄▄█  █▄█▄█   █▄█▄▄▄█▄█  █▄▄█▄▄▄▄▄▄█ 
";

const PARTY: &str = "
-----------------------------------------------------
   _                             .-.
  / )  .-.    ___          __   (   )
 ( (  (   ) .'___)        (__'-._) (
  \\ '._) (,'.'               '.     '-.
   '.      /  \"\\               '    -. '.
     )    /   \\ \\   .-.   ,'.   )  (  ',_)    _
   .'    (     \\ \\ (   \\ . .' .'    )    .-. ( \\
  (  .''. '.    \\ \\|  .' .' ,',--, /    (   ) ) )
   \\ \\   ', :    \\    .-'  ( (  ( (     _) (,' /
    \\ \\   : :    )  / _     ' .  \\ \\  ,'      /
  ,' ,'   : ;   /  /,' '.   /.'  / / ( (\\    (
  '.'      \"   (    .-'. \\       ''   \\_)\\    \\
                \\  |    \\ \\__             )    )
              ___\\ |     \\___;           /  , /
             /  ___)                    (  ( (
             '.'                         ) ;) ;
                                        (_/(_/
----------------------------------------------------
";

trait FromLetter {
    fn from_letter(l: char) -> Result<Colors, ColorParsingError>;
    fn to_char(&self) -> char;
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

    fn to_char(&self) -> char {
        match self {
            Colors::Blue =>'B',
            Colors::Red => 'R',
            Colors::Purple => 'P',
            Colors::Green => 'G',
            Colors::Yellow => 'Y',
            Colors::Pink => 'I',
            Colors::Cyan => 'C',
            Colors::Magenta => 'M',
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
        println!("Error setting console color: {}", e.to_string());
    } else if let Err(e) = write!(&mut stdout, " ⦿ ") {
        println!("Error occured writing: {}", e.to_string());
    }

    if let Err(e) = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))) {
        println!("Error setting color back to white: {}", e);
    }
}

fn print_color_choices() {
    let colors = vec![
        Colors::Blue,
        Colors::Cyan,
        Colors::Green,
        Colors::Magenta,
        Colors::Pink,
        Colors::Purple,
        Colors::Red,
        Colors::Yellow,
    ];

    for color in colors {
        fancy_print_color(&color);
        println!(" <==> {}", color.to_char());
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

fn pretty_print_guess(guess: &[Colors]) {
    print!("[");
    for color in guess {
        fancy_print_color(color);
        print!(" ");
    }
    println!("]");
}

fn number_of_well_placed_pawns(secret: &[Colors], guess: &[Colors]) -> i32 {
    let mut counter: i32 = 0;
    for n in 0..secret.len() {
        if secret[n] == guess[n] {
            counter += 1;
        }
    }

    return counter;
}

fn number_of_not_well_placed_pawns(secret: &[Colors], guess: &[Colors]) -> i32 {
    let mut counter: i32 = 0;
    for i in 0..secret.len() {
        for j in 0..secret.len() {
            if i != j && secret[i] == guess[j] {
                counter += 1;
            }
        }
    }

    return counter;
}

fn generate_random_secret() -> Vec<Colors> {
    
    
    let mut res = Vec::new();

    for _ in 0..5 {
        let choice: usize = rand::random::<usize>() % 8;

        let color = match choice {
            0 => Colors::Blue,
            1 => Colors::Red,
            2 => Colors::Purple,
            3 => Colors::Green,
            4 => Colors::Yellow,
            5 => Colors::Pink,
            6 => Colors::Cyan,
            7 => Colors::Magenta,
            _ => Colors::Blue // is never going to happen. how do I set this properly ? 
        };

        res.push(color);
    }

    res
}

fn main() {

    let mut stop_flag = false;

    while !stop_flag {
        println!("{}", BANNER);
        pause_terminal();

        let mut tries: u8 = 0;

        let secret = generate_random_secret();

        loop {
            clear_terminal();

            // uncomment for god mode
            // pretty_print_guess(&secret); 

            print_color_choices();
            println!("Please input 5 letters using their corresponding letter above.");
            let stdin = io::stdin();

            let mut buffer = String::new();

            stdin
                .read_line(&mut buffer)
                .ok()
                .expect("An error ocured reading input.");

            buffer.pop();

            let input = parse_input(&buffer);
            
            clear_terminal();

            if let Ok(array) = input {
                let well_placed = number_of_well_placed_pawns(&secret, &array);

                if well_placed < 5 {
                    pretty_print_guess(&array);
                    println!("You got {} well placed pawns !", well_placed);
                    println!("You got {} wrong placed pawns !", number_of_not_well_placed_pawns(&secret, &array));
                    tries += 1;

                } else {
                    println!("{}", PARTY);
                    println!("Nailed it ! and in only {} times !", tries + 1);
                    break;
                }
                
                
            } else {
                println!("{}", input.err().unwrap())
            }

            pause_terminal();
        }
        prompt_stop(&mut stop_flag);
    }
}
