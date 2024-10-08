use std::io;
use std::num::ParseIntError;

fn main() {
    loop {
        let number = match get_number_from_user() {
            Ok(num) => num,
            Err(e) => {
                println!("Failed to parse the number: {e}");
                continue;
            }
        };
        print_num(number);
    }
}

fn get_number_from_user() -> Result<i64, ParseIntError> {
    let mut input = String::new();
    println!("Please enter a number (0x/0b/0o):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();

    parse_user_input(&input)
}

fn parse_user_input(input: &String) -> Result<i64, ParseIntError> {
    let result = match &input[0..2] {
        "0x" => i64::from_str_radix(input.strip_prefix("0x").unwrap(), 16)?,
        "0b" => i64::from_str_radix(input.strip_prefix("0b").unwrap(), 2)?,
        "0o" => i64::from_str_radix(input.strip_prefix("0o").unwrap(), 8)?,
        _ => input.parse::<i64>()?,
    };
    Ok(result)
}

/// Returns a binary representation of a number with groups of 4 digits separated by a space
///
/// # Arguments
///
/// * `number`: The number to convert
///
/// returns: String
///
fn group_binary(number: i64) -> String {
    let binary = format!("{:b}", number);
    let mut grouped = String::new();

    let padding = 4 - (binary.len() % 4);
    if padding != 4 {
        grouped.push_str(&"0".repeat(padding));
    }
    grouped.push_str(&binary);

    let chunks: Vec<String> = grouped
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| chunk.iter().collect())
        .collect();

    chunks.join(" ")
}

fn prune_bin(hex_str: String) -> String {
    if !(&hex_str).starts_with("1") {
        hex_str
    } else {
        hex_str.replace(" 1111", "").to_string()
    }
}

fn format_binary(number: i64) -> String {
    prune_bin(group_binary(number))
}

fn print_num(number: i64) {
    println!("Your number in different bases:");
    println!("Dec:\t  {}", number);
    println!("Hex:\t{:#X}", number);
    println!("Bin:\t0b{}", format_binary(number));
    println!("Oct:\t{:#o}", number);
    println!("{}", "-".repeat(20));
}
