use clap::Parser;
use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    length: u32, //lenght of the password
    #[arg(short = 'c', long = "capitals")]
    capitals: Option<u32>,
    #[arg(short = 'd', long = "digits")]
    digits: Option<u32>,
    #[arg(short = 's', long = "symbols")]
    symbols: Option<u32>,
}

#[derive(Debug)]
struct PasswordRequirements {
    lowercase: u32,
    capitals: u32,
    digits: u32,
    symbols: u32,
}

fn get_password_requirements(args: Args) -> Option<PasswordRequirements> {
    let capitals: u32 = args.capitals.unwrap_or(0);
    let digits: u32 = args.digits.unwrap_or(0);
    let symbols: u32 = args.symbols.unwrap_or(0);
    let sum: u32 = capitals + digits + symbols;

    if sum > args.length {
        None
    } else {
        Some(
            PasswordRequirements {
                lowercase: args.length - sum,
                capitals,
                digits,
                symbols
            }
        )
    }
}

fn generate_string_from_charset(charset: &&[u8], length: u32) -> String {
    let mut rng = rand::thread_rng();
    let charset_len = (*charset).len();
    let random_string: String = (0..length).map(|_| {
        let index = rng.gen_range(0..charset_len);
        (*charset)[index] as char
    })
    .collect();
    random_string
}

fn generate_password_from_requirements(requirements: PasswordRequirements) -> String {
    let all_lowercase: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let all_uppercase: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let all_digits: &[u8] = b"0123456789";
    let authorized_symbols: &[u8] = b"!@#$%^&*";
    let mut concatenated_random_string: String = String::new();

    let mut rng = rand::thread_rng();

    if requirements.lowercase != 0 {
        let generated = generate_string_from_charset(&all_lowercase, requirements.lowercase);
        concatenated_random_string = format!("{}{}", concatenated_random_string, generated);
    }

    if requirements.capitals != 0 {
        let generated = generate_string_from_charset(&all_uppercase, requirements.capitals);
        concatenated_random_string = format!("{}{}", concatenated_random_string, generated);
    }

    if requirements.digits != 0 {
        let generated = generate_string_from_charset(&all_digits, requirements.digits);
        concatenated_random_string = format!("{}{}", concatenated_random_string, generated);
    }

    if requirements.symbols != 0 {
        let generated = generate_string_from_charset(&authorized_symbols, requirements.symbols);
        concatenated_random_string = format!("{}{}", concatenated_random_string, generated);
    }


    let mut chars: Vec<char> = concatenated_random_string.chars().collect(); // Convert String into a vector to shuffle

    chars.shuffle(&mut rng);
    
    chars.into_iter().collect() // convert back into a String
}

fn main() {
    let args = Args::parse();

    match get_password_requirements(args) {
        Some(requirements) => println!("{}", generate_password_from_requirements(requirements)),
        _ => println!("Incorrect requirements.")
    }
}
