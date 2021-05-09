extern crate rand;
use rand::Rng;
use std::io;
use std::str::FromStr;
struct Color {r:i32, g:i32, b:i32}

const RED: Color = Color {r:255, g:0, b:0};
const GREEN: Color = Color {r:255, g:0, b:0};
const BLUE: Color = Color {r:255, g:0, b:0};
const BROWN: Color = Color {r:255, g:0, b:0};
const BLACK: Color = Color {r:255, g:0, b:0};
const WHITE: Color = Color {r:255, g:0, b:0};
const PINK: Color = Color {r:255, g:0, b:0};
const GREY: Color = Color {r:255, g:0, b:0};

#[derive(Debug)]
pub enum enumColor {
    RED ,
    GREEN,
    BLUE,
    BROWN,
    BLACK,
    WHITE,
    PINK,
    GREY,
}

fn fancy_pring_guess(guess: &Vec<enumColor>)  {
    let mut res ="None";

    for _n in 0..5{
        let current_color = &guess[_n];
        let mut res ="None";

        match current_color{
            enumColor::RED => res="R",
            enumColor::GREEN => res="Green",
            enumColor::BLUE => res="Blu",
            enumColor::BROWN => res="Br",
            enumColor::BLACK => res="Bla",
            enumColor::WHITE => res="W",
            enumColor::PINK => res="P",
            enumColor::GREY => res="G",
            _ => res="G",

        }
    }


}

fn getColor(id: usize, guess: &Vec<enumColor>)-> String  {
        let mut res ="None";

        let current_color = &guess[id];

        match current_color{
            enumColor::RED => res="R",
            enumColor::GREEN => res="Green",
            enumColor::BLUE => res="Blu",
            enumColor::BROWN => res="Br",
            enumColor::BLACK => res="Bla",
            enumColor::WHITE => res="W",
            enumColor::PINK => res="P",
            enumColor::GREY => res="G",
            _ => res="G",

        }

        return res.to_owned();

}




fn recuperer_entree_utilisateur() -> String { // elle ne prend rien en entrée et retourne un Option<isize> (dans le cas où ça ne fonctionnerait pas)
    let mut entree = String::new();

    io::stdin()
        .read_line(&mut entree)
        .expect("Échec de la lecture de l'entrée utilisateur");

    println!("Votre couleur : {}", entree);
    return entree.to_owned().trim().to_string();
}




fn number_of_well_placed_pawns(secret: &[enumColor], guess: &[enumColor]) -> i32{


    return 3;
}

fn number_of_not_well_placed_pawns(secret: usize) -> usize{

    return 5-secret;
}


fn main() {

    let mut endGame = false;

    let mut guess = Vec::new();
    let mut cptFind = 0;

    for n in 0..5{

        let valAleatoire = rand::thread_rng().gen_range(1..8);
        
        match valAleatoire{
            1 => guess.push(enumColor::RED),
            2 => guess.push(enumColor::GREEN),
            3 => guess.push(enumColor::BLUE),
            4 => guess.push(enumColor::BROWN),
            5 => guess.push(enumColor::BLACK),
            6 => guess.push(enumColor::WHITE),
            7 => guess.push(enumColor::PINK),
            8 => guess.push(enumColor::GREY),
            _ => guess.push(enumColor::RED),

        }
    
        println!("Hello, world! {:?}", guess);

    }





    while(!endGame){

        println!("Bonjour, saisissez une couleur: parmis P(pink), R(red), W(white), G(grey), Bla(black), Br(brown), Blu(blue) et Green(green)");
       
        let mut saisie = recuperer_entree_utilisateur();

       
    

        if(saisie.eq(&getColor(cptFind, &guess))){
            cptFind = cptFind+1;
            println!("Bravo, vous avez trouvé {} pions",cptFind);

            println!("Il reste {} pions", number_of_not_well_placed_pawns(cptFind));

            
        }else{
            println!("Raté");
        }

        if(cptFind == 5){
            endGame = true;
            println!("Vous avez gagné la partie");
        }

        fancy_pring_guess(&guess);

    }
    
}
    




