use std::io;

fn main() {
    // Read the decimal number input from the user
    let mut decimal = String::new();
    println!("Enter a decimal number:");
    io::stdin()
        .read_line(&mut decimal)
        .expect("Failed to read line");
    let decimal: u32 = decimal.trim().parse().expect("Invalid decimal number");

    // Convert the decimal number to binary
    let mut binary_digits = Vec::new();
    let mut n = decimal;
    while n > 0 {
        let rem = n % 2;
        binary_digits.push(rem);
        n /= 2;
    }
    // Reverse the order of the binary digits
    binary_digits.reverse();
    // Print the binary digits to the console
    println!("Binary equivalent: {:?}", binary_digits);
}