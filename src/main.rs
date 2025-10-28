use rand::Rng;

fn main() {
    let password = generate_password(12, false);
    println!("🔐 Generated password: {}", password);
}

fn generate_password(length: usize, include_symbols: bool) -> String {
    let mut charset = String::from(
        "abcdefghijklmnopqrstuvwxyz\
         ABCDEFGHIJKLMNOPQRSTUVWXYZ\
         0123456789"
    );
    
    if include_symbols {
        charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?");
    }
    
    let charset: Vec<char> = charset.chars().collect();
    let mut rng = rand::thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx]
        })
        .collect()
}
