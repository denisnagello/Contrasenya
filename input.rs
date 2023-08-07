use std::io;
use std::io::Write;

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

pub fn read_yes_no(prompt: &str) -> bool {
    loop {
        let input: String = read_input(prompt);
        match input.to_lowercase().as_str() {
            "yes" => return true,
            "no" => return false,
            _ => println!("Please enter 'yes' or 'no'."),
        }
    }
}

pub fn read_usize_input(prompt: &str, min: usize, max: usize) -> usize {
    loop {
        let input = read_input(prompt);
        match input.parse() {
            Ok(val) if val >= min && val <= max => return val,
            Ok(_) => println!("Invalid input. Please enter a valid number between {} and {}.", min, max),
            Err(_) => println!("Invalid input. Please enter a valid positive number."),
        }
    }
}