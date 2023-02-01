use std::io;

fn main() {
    println!("Welcome to the calculator!");

    loop {
        println!("Enter the expression you want to calculate (e.g. 2 + 2):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            break;
        }

        let parts: Vec<&str> = input.split(" ").collect();
        if parts.len() != 3 {
            println!("Invalid expression");
            continue;
        }

        let a = parts[0].parse::<f64>().unwrap();
        let b = parts[2].parse::<f64>().unwrap();

        let result = match parts[1] {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _ => {
                println!("Invalid operator");
                continue;
            }
        };

        println!("{}", result);
    }

    println!("Goodbye!");
}
