use rand::Rng;
use std::io::{self, Write};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess the number between 1 and 100, enter x to quit.");

    loop {
        print!("Input your guess: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        // let guess = guess.trim().parse::<u32>().unwrap();
        let guess = guess.trim();
        if guess == "x" {
            break;
        }
        let guess = guess.parse::<u32>().unwrap();
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin().read_line(&mut guess).expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 continue;
//             }
//         };

//         println!("You guessed: {guess}");

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }