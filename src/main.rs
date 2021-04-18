use ansi_term::Colour;
use std::io;

fn main() -> io::Result<()> {
    #[derive(Debug, PartialEq)]
    enum Color {
        Cyan,
        Green,
        Blue,
        Purple,
        Red,
        White,
        Yellow,
    }

    fn fancy_print_guess(guess: &[Color]) {
        for color in guess {
            match color {
                Color::Yellow => print!("{}", Colour::Yellow.paint("Y")),
                Color::Cyan => print!("{}", Colour::Cyan.paint("C")),
                Color::Green => print!("{}", Colour::Green.paint("G")),
                Color::Blue => print!("{}", Colour::Blue.paint("B")),
                Color::Purple => print!("{}", Colour::Purple.paint("P")),
                Color::Red => print!("{}", Colour::Red.paint("R")),
                Color::White => print!("{}", Colour::White.paint("W")),
            }
        }
        println!("");
    }

    fn parse_vec_into_string(guess: &[Color]) -> String {
        let mut result = String::new();
        for color in guess {
            match color {
                Color::Yellow => result.push('Y'),
                Color::Cyan => result.push('C'),
                Color::Green => result.push('G'),
                Color::Blue => result.push('B'),
                Color::Purple => result.push('P'),
                Color::Red => result.push('R'),
                Color::White => result.push('W'),
            }
        }
        return result;
    } 
    
    fn parse_string_into_vec(input: String) -> Vec<Color>{
        let mut colors:Vec<Color> = vec![];
        for aa in input.chars() { 
            match aa {
                'Y' => colors.push(Color::Yellow),
                'C' => colors.push(Color::Cyan),
                'G' => colors.push(Color::Green),
                'B' => colors.push(Color::Blue),
                'P' => colors.push(Color::Purple),
                'R' => colors.push(Color::Red),
                'W' => colors.push(Color::White),
                '\n' => print!("word: "),
                _ => {
                    println!("Wrong input");
                    return vec![];
                }
            }
        }
        fancy_print_guess(&colors);
        return colors;
    }

    let guess:Vec<Color> = vec![Color::Cyan,Color::Green,Color::Blue,Color::Yellow,Color::Purple];
    let mut nb_tries = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let mut results :Vec<Color> = parse_string_into_vec(input);
        if results.len() > 0 {
            nb_tries += 1;
            println!("number of tries: {}", nb_tries);
        }
    }


    Ok(())
}
