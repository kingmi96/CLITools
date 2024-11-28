use clap::Args;
use rand::Rng;
use std::iter;

#[derive(Args)]
pub struct PasswordOptions {
    #[arg(short = 'l', long, default_value_t = 12, help = "Length of the password")]
    length: usize,

    #[arg(short = 's', long, help = "Include special characters")]
    special: bool,

    #[arg(short = 'n', long, help = "Include numbers")]
    numbers: bool,

    #[arg(short = 'u', long, help = "Include uppercase letters")]
    uppercase: bool,

    #[arg(short = 'c', long, help = "Include lowercase letters")]
    lowercase: bool,
}

pub fn run_password_generator(options: PasswordOptions) {
    let password = generate_password(options);
    println!("Generated Password: {}", password);
}

fn generate_password(options: PasswordOptions) -> String {
    let mut rng = rand::thread_rng();
    let mut characters = String::new();

    if options.lowercase {
        characters.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if options.uppercase {
        characters.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if options.numbers {
        characters.push_str("0123456789");
    }
    if options.special {
        characters.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?/`~");
    }

    if characters.is_empty() {
        characters.push_str("abcdefghijklmnopqrstuvwxyz"); // Default to lowercase letters
    }

    let password: String = iter::repeat_with(|| {
        let idx = rng.gen_range(0..characters.len());
        characters.chars().nth(idx).unwrap()
    })
    .take(options.length)
    .collect();

    password
}
