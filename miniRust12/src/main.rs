use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a filename");
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut word_count = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        for _word in line.split_whitespace() {
            word_count += 1;
        }
    }

    println!("Total word count: {}", word_count);
}