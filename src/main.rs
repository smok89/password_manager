use clap::{Parser, Subcommand};
use rand::Rng;
use rand::seq::SliceRandom;
use inquire::{CustomType, Confirm};
use clipboard::{ClipboardContext, ClipboardProvider};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug)]
struct RequirementsArgs {
    length: u32, //lenght of the password
    #[arg(short = 'c', long = "capitals")]
    capitals: Option<u32>,
    #[arg(short = 'd', long = "digits")]
    digits: Option<u32>,
    #[arg(short = 's', long = "symbols")]
    symbols: Option<u32>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Generate(RequirementsArgs),
    Auto,
}

#[derive(Debug)]
struct PasswordRequirements {
    lowercase: u32,
    capitals: u32,
    digits: u32,
    symbols: u32,
}

fn get_password_requirements(length: u32, capitals: u32, digits: u32, symbols: u32) -> Option<PasswordRequirements> {
    let sum: u32 = capitals + digits + symbols;

    if sum > length {
        None
    } else {
        Some(
            PasswordRequirements {
                lowercase: length - sum,
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
    (0..length).map(|_| {
        let index = rng.gen_range(0..charset_len);
        (*charset)[index] as char
    })
    .collect()
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

fn prompt_user_for_password_requirements() -> PasswordRequirements {
    let length: u32 = CustomType::new("Enter password length")
        .with_default(12)
        .prompt()
        .expect("Failed to get input");

    // Compute default value for uppercase letters, symbols and digits
    let default_partition = (length as f64 / 4.0).floor() as u32;

    let capitals: u32 = CustomType::new("Enter the number of capital letters")
        .with_default(default_partition)
        .prompt()
        .expect("Failed to get input");

    let digits: u32 = CustomType::new("Enter the number of digits")
        .with_default(default_partition)
        .prompt()
        .expect("Failed to get input");

    let symbols: u32 = CustomType::new("Enter the number of symbols")
        .with_default(default_partition)
        .prompt()
        .expect("Failed to get input");

    get_password_requirements(length, capitals, digits, symbols)
        .expect("Incorrect requirements: the sum of capitals, digits, and symbols exceeds the total length.")
}

fn copy_to_clipboard(password: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Failed to access clipboard.");
    ctx.set_contents(password).expect("Failed to copy to clipboard.");
}

fn compute_and_print_password(requirements: PasswordRequirements) {
    let password = generate_password_from_requirements(requirements);
                    println!("Successully generated a password: {}", password);
                    if Confirm::new("Do you wish to copy this password to the clipboard?")
                        .with_default(true)
                        .prompt()
                        .unwrap()
                    {
                        copy_to_clipboard(password);
                        println!("Password copied to clipboard.")
                    }
}

fn get_auto_requirements() -> PasswordRequirements {
    PasswordRequirements { 
        lowercase: 8,
        capitals: 8,
        digits: 8,
        symbols: 8,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Generate(args)) => {
            // using the values provided directly when invoking the program
            let caps = args.capitals.unwrap_or(0);
            let digs = args.digits.unwrap_or(0);
            let syms = args.symbols.unwrap_or(0);

            match get_password_requirements(args.length, caps, digs, syms) {
                Some(requirements) => {compute_and_print_password(requirements) },
                None => println!("Incorrect requirements: the sum of capitals, digits, and symbols exceeds the total length.")
            }
        },
        Some(Commands::Auto) => {
            let requirements = get_auto_requirements();
            compute_and_print_password(requirements);
        }
        None => {
            // calling the inquire interface to get the values interactively
            compute_and_print_password(prompt_user_for_password_requirements());     
        }
    }
}
