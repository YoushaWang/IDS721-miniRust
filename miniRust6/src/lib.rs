use rand::*;

// A function that generates a random password
pub fn generate_password(length: usize, simple: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    let mut i = 0;
    let low = if simple {48} else {33};
    let high = if simple {57} else {126};
    while i < length {
        let random_number = rng.gen_range(0..=127);
        if random_number >= low && random_number <= high {
            password.push(random_number as u8 as char);
            i += 1;
        }
    }
    password
}