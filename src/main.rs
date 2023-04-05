use rand::Rng;

fn main() {
    // Define the characters that can be used in the password
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+";

    // Generate a random password of length 12
    let password: String = (0..12)
        .map(|_| {
            let index = rand::thread_rng().gen_range(0..charset.len());
            charset[index] as char
        })
        .collect();

    println!("{}", password);
}