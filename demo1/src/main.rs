fn main() {
    let mut grades = Vec::new();
    let mut credit_hours = Vec::new();

    // Get grades and credit from user
    for i in 1.. {
        println!("Enter grade for class {} (or 'q' to quit)", i);
        let mut grade = String::new();
        std::io::stdin().read_line(&mut grade).unwrap();
        grade = grade.trim().to_uppercase();

        if grade == "Q" {
            break;
        }

        let grade = match grade.as_str() {
            "A+" => 4.0,"A" => 4.0,"A-" => 3.7,
            "B+" => 3.3,"B" => 3.0,"B-" => 2.7,
            "C+" => 2.3,"C" => 2.0,"C-" => 1.7,
            "F" => 0.0,"NC"=>0.0,
            _ => {
                println!("Invalid grade, try again");
                continue;
            }
        };

        println!("Enter credit for this course {}:", i);
        let mut credit_hour = String::new();
        std::io::stdin().read_line(&mut credit_hour).unwrap();
        let credit_hour: f32 = match credit_hour.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid credit, try again");
                continue;
            }
        };

        grades.push(grade);
        credit_hours.push(credit_hour);
    }

    // Calculate GPA
    let mut total_quality_points = 0.0;
    let mut total_credit_hours = 0.0;
    for i in 0..grades.len() {
        total_quality_points += grades[i] * credit_hours[i];
        total_credit_hours += credit_hours[i];
    }

    let gpa = total_quality_points / total_credit_hours;
    println!("GPA: {:.2}", gpa);
}
// //A command-line tool that plays Marco Polo
// use clap::Parser;

// #[derive(Parser)]
// #[clap(version = "1.0", author = "Noah Gift", about = "A Marco Polo game.")]
// struct Cli {
//     #[clap(subcommand)]
//     command: Option<Commands>,
// }

// #[derive(Parser)]
// enum Commands {
//     #[clap(version = "1.0", author = "Noah Gift", about = "A Marco Polo game.")]
//     Marco {
//         #[clap(short, long)]
//         name: String,
//     },
// }

// // This is the main function
// // hello::marco_polo(&name)
// fn main() {
//     let args = Cli::parse();
//     match args.command {
//         Some(Commands::Marco { name }) => {
//             println!("{}", hello::marco_polo(&name));
//         }
//         None => println!("No command was used"),
//     }
// }
