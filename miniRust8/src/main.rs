use rand::Rng;
use std::io::{self, Write};

fn main() {
    loop {
        let fortune = generate_fortune();
        println!("{}", fortune);

        print!("Press Enter to generate a new message, or type 'quit' to exit...");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "quit" {
            break;
        }
    }
}

fn generate_fortune() -> String {
    const FORTUNES: &[&str] = &[
        "A bird in the hand is worth two in the bush.",
        "A penny saved is a penny earned.",
        "All work and no play makes Jack a dull boy.",
        "Don't count your chickens before they're hatched.",
        "Every dog has its day.",
        "Every cloud has a silver lining.",
        "If you can't stand the heat, get out of the kitchen.",
        "It takes two to tango.",
        "Life is like a box of chocolates; you never know what you're gonna get.",
        "Practice makes perfect.",
        "When in Rome, do as the Romans do.",
        "You can't judge a book by its cover.",
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..FORTUNES.len());
    FORTUNES[index].to_owned()
}
