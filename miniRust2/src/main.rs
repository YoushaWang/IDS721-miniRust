use rand::Rng;
use std::io;

fn main() {
    println!("Rock, Paper, Scissors");
    println!("---------------------");
    println!("Enter 'x' to exit");

    loop {
        let mut input = String::new();
        println!("Enter your choice (rock, paper, scissors): ");

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "x" {
            break;
        }

        let computer = get_computer_choice();
        let result = eval(input, computer);

        match result {
            Ok(value) => println!("Result: {}", value),
            Err(error) => println!("Error: {}", error),
        }
    }
}

fn get_computer_choice() -> String {
    let choices = vec!["rock", "paper", "scissors"];
    let choice = rand::thread_rng().gen_range(0, 3);

    choices[choice].to_string()
}

fn eval(player: &str, computer: String) -> Result<String, &str> {
    let result = match (player, computer.as_str()) {
        ("rock", "scissors") => "Player wins!",
        ("paper", "rock") => "Player wins!",
        ("scissors", "paper") => "Player wins!",
        ("rock", "rock") => "Draw!",
        ("paper", "paper") => "Draw!",
        ("scissors", "scissors") => "Draw!",
        (_, _) => "Computer wins!",
    };

    Ok(format!("{} vs. {}: {}", player, computer, result))
}