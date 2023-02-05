use std::io;
use std::vec::Vec;

fn main() {
    let mut todo_list: Vec<String> = Vec::new();
    loop {
        println!("What would you like to do?");
        println!("1. Add a task");
        println!("2. Complete a task");
        println!("3. List all tasks");
        println!("4. Quit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
        match choice {
            1 => {
                println!("Enter a task:");
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                todo_list.push(task.trim().to_owned());
                println!("Task added");
            }
            2 => {
                if todo_list.is_empty() {
                    println!("No tasks to complete");
                } else {
                    println!("Enter the index of the task to complete:");
                    let mut index = String::new();
                    io::stdin().read_line(&mut index).unwrap();
                    let index = match index.trim().parse::<usize>() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid input");
                            continue;
                        }
                    };
                    if index >= todo_list.len() {
                        println!("Invalid index");
                    } else {
                        todo_list.remove(index);
                        println!("Task completed");
                    }
                }
            }
            3 => {
                if todo_list.is_empty() {
                    println!("No tasks to show");
                } else {
                    println!("Todo List:");
                    for (index, task) in todo_list.iter().enumerate() {
                        println!("{}. {}", index, task);
                    }
                }
            }
            4 => {
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}
