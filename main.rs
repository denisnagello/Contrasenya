use std::io::Write;

mod generator;
mod input;

fn main() {
    println!("Password Generator in Rust");

    const MIN_PASSWORD_LENGTH: usize = 5;
    const MAX_PASSWORD_LENGTH: usize = 128;

    let length: usize = input::read_usize_input("Length of the password: ", MIN_PASSWORD_LENGTH, MAX_PASSWORD_LENGTH);

    let include_lowercase: bool = input::read_yes_no("Include lowercase letters? (yes/no): ");
    let include_uppercase: bool = input::read_yes_no("Include uppercase letters? (yes/no): ");
    let include_digits: bool = input::read_yes_no("Include digits? (yes/no): ");
    let include_special_chars: bool = input::read_yes_no("Include special characters? (yes/no): ");

    let password: String = generator::generate_password(length, include_lowercase, include_uppercase, include_digits, include_special_chars);
    println!("Generated Password: {}", password);
}