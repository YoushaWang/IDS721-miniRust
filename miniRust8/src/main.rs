use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration};
// use chrono::{DateTime, Local, TimeZone};

fn main() {
    println!("Welcome to the Rust Digital Clock and Timer!");

    loop {
        println!("Please select an option:");
        println!("1. Digital Clock");
        println!("2. Timer");
        println!("3. Quit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");

        match option.trim() {
            "1" => digital_clock(),
            "2" => timer(),
            "3" => break,
            _ => println!("Invalid option!"),
        }
    }

    println!("Thanks for using the Rust Digital Clock and Timer!");
}

fn digital_clock() {
    loop {

        let current_time = chrono::Local::now().with_timezone(&chrono_tz::America::New_York).format("%H:%M:%S");
        println!("\r{}", current_time);
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}

fn timer() {
    println!("Please enter the duration of the timer in seconds:");
    let mut duration_str = String::new();
    io::stdin().read_line(&mut duration_str).expect("Failed to read line");

    let duration: u64 = match duration_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid duration! Defaulting to 10 seconds.");
            10
        }
    };

    for i in (0..duration).rev() {
        print!("\r{} ", i);
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }

    println!("\rTimer complete!");
}
