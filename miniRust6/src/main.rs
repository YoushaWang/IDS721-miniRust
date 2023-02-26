use rand::Rng;

fn main() {
    let password = generate_password(12);
    println!("Your new password is: {}", password);
}

fn generate_password(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789)(*&^%$#@!";

    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let index = rng.gen_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect();

    password
}
