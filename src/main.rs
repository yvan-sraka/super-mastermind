fn main() {
    use ansi_term::Colour::{Red,Yellow,Green,Purple,Blue};
    use std::io;

    #[derive(Debug,PartialEq,Copy,Clone)]
    enum Color {Red,Orange,Blue,Yellow,Green,Cyan,White,Purple}

    let all_color = vec![Color::Red,Color::Green,Color::Yellow,Color::Orange,Color::Blue,Color::Cyan,Color::Purple,Color::White];

    println!("Entrée le nombre de couleur voulus :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number_in_secret: i32 = input.trim().parse().unwrap();

    let secret_color = gen_rnd_color(&all_color,number_in_secret);

    
    println!("Entrée une serie de couleurs :");

    loop {
        let mut guess = Vec::new();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Échec de la lecture de l'entrée utilisateur");

        for i in 0..input.len() - 1
        {
            let mut err = 0;
            match input.chars().nth(i).unwrap() {
                'R' => guess.push(Color::Red),
                'O' => guess.push(Color::Orange),
                'G' => guess.push(Color::Green),
                'C' => guess.push(Color::Cyan),
                'P' => guess.push(Color::Purple),
                'Y' => guess.push(Color::Yellow),
                'B' => guess.push(Color::Blue),
                'W' => guess.push(Color::White),
                _ => {
                    println!("error");
                    err = 1;
                }
            }

            if err == 1{
                break;
            }
        }
        println!("====================================================================================");
        fancy_print_guess(&guess);
        let number_well = number_of_well_placed_pawns(&secret_color,&guess);
        if  number_well == number_in_secret
        {
            println!("Congratulation you win");
            break;
        }
        else
        {
            println!("You have {} color well place",number_well);
            println!("You have {} color not well place, Retry",number_of_not_well_placed_pawns(&secret_color,&guess) - number_well );
        }
    }
    
    fn fancy_print_guess(guess: &[Color]){
        for color_in_vector in guess{           
             match color_in_vector {
                &Color::Red => print!("{}",Red.paint("R")),
                &Color::Blue => print!("{}",Blue.paint("B")),
                &Color::White => print!("W"),
                &Color::Cyan => print!("{}",Blue.paint("C")),
                &Color::Purple => print!("{}",Purple.paint("P")),
                &Color::Yellow => print!("{}",Yellow.paint("Y")),
                &Color::Green => print!("{}",Green.paint("G")),
                &Color::Orange => print!("{}",Red.paint("O")),  
            }
        }
        println!("");
    }

    
   fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32
    {
        let mut number_of_well_placed = 0;
        for _i in 0..secret.len()
        {
            if secret[_i] == guess[_i]
            {
                number_of_well_placed = number_of_well_placed + 1;
            }
        }
        return number_of_well_placed;
    }

    fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32
    {
        let mut color_not_well = 0;
         for color_in_secret in secret{           
             for color_in_guess in guess{           
                if color_in_secret == color_in_guess
                {color_not_well = color_not_well + 1;}
            }
        }
        return color_not_well;
    }

    fn gen_rnd_color(all_color: &[Color],number_in_secret: i32) -> Vec<Color>{
        let mut secret: Vec<Color> = Vec::new();
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        for _i in 0..number_in_secret
        {
            secret.push(all_color[rng.gen_range(0..all_color.len())])
        }
        return secret;
    }
}
