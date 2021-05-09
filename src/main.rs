
#[derive(PartialEq)]
enum Color {
    Green,
    Red,
    White,
    Black,
    Blue,
    Orange,
    Purple,
    Grey,
}

fn fancy_print_guess(guess: &[Color]) {

    let mut combi: String= String::new();

    for elem in guess {
        combi += match elem {
            Color::White => "W",
            Color::Red => "R",
            Color::Purple => "P",
            Color::Orange => "O",
            Color::Grey => "G",
            Color::Green => "g",
            Color::Blue => "b",
            Color::Black => "B",
        }
    }
    println!("{}",format!("{}", combi));

}

fn main() {

    let guess = vec![Color::Blue,
                    Color::Green,
                    Color::Grey,
                    Color::Orange,
                    Color::Purple
    ];

    let mut count = 0;


    loop {
        count+=1;
        let mut line = String::new();
        println!("Enter your combination :");
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
     
        let mut answer = Vec::new();
     
        for elem in line.chars() {
            match elem {
             'W' => answer.push(Color::White),
             'R' => answer.push(Color::Red),
             'P' => answer.push(Color::Purple),
             'O' => answer.push(Color::Orange),
             'G' => answer.push(Color::Grey),
             'g' => answer.push(Color::Green),
             'b' => answer.push(Color::Blue),
             'B' => answer.push(Color::Black),
             _ => {
                    println!("Error input");
                    break;
                }
            }; 
        };
        let mut validation = true;
        
        if answer.len()!= guess.len() {
            validation= false;
        } else {
            for i in 0..answer.len() {
                if answer[i] != guess[i] {
                    validation=false;
                    println!("Try again");
                    break;
                }
            }
            if validation == true {
                println!("Congratulations !!! you win and your total try is {}", count);
                break;
            }
        }
    }
    
}

