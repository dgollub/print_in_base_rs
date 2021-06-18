fn get_user_input(prompt: &str) -> String {
    use std::io::{stdin, stdout, Write};

    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Could not read input from tty. Reason: {}", err);
        }
    }

    input.trim().to_string()
}

const ALPHABET: &str = "0123456789abcdefghijklmnopqrstuvwxyz";

/// Convert a single number into a char
/// Input 0..9: output 0..9
/// Input 10..32: output a..z
fn convert_number_to_char(c: usize) -> Option<char> {
    ALPHABET.chars().nth(c)
}

fn main() {
    println!("Print In Base");

    let number = get_user_input("Please enter a number: ");
    let base = get_user_input("Please enter a base: ");

    let number = number.parse::<usize>().expect("Expected a integer number.");
    let base = base.parse::<usize>().expect("Expected a integer number.");

    println!("Number: {}", number);
    println!("Base  : {}", base);

    let mut n = number;
    let mut result = String::new();

    while n > 0 {
        let x = convert_number_to_char(n % base).unwrap();
        result.push(x);
        n /= base;
    }

    // TODO(dkg): try to rewrite so we avoid the rev().collect() call
    println!("{}", result.chars().rev().collect::<String>());
    println!("Done");
}
