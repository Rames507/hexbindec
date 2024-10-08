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
    let prefix: &str = &input[0..2];
    let result = match prefix {
        "0x" => i64::from_str_radix(input.strip_prefix(prefix).unwrap(), 16)?,
        "0b" => i64::from_str_radix(input.strip_prefix(prefix).unwrap(), 2)?,
        "0o" => i64::from_str_radix(input.strip_prefix(prefix).unwrap(), 8)?,
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

fn remove_leading_ones(binary_string: String) -> String {
    if !(&binary_string).starts_with("1") {
        binary_string
    } else {
        binary_string.replace(" 1111", "").to_string()
    }
}

fn format_binary(number: i64) -> String {
    remove_leading_ones(group_binary(number))
}

fn print_num(number: i64) {
    println!("Your number in different bases:");
    println!("Dec:\t  {}", number);
    println!("Hex:\t{:#X}", number);
    println!("Bin:\t0b{}", format_binary(number));
    println!("Oct:\t{:#o}", number);
    println!("{}", "-".repeat(20));
}
