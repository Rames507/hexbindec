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
    if input.len() < 2 {
        return Ok(input.parse()?);
    }
    let prefix: &str = &input[0..2];
    let result = match prefix {
        "0x" => i64::from_str_radix(input.strip_prefix(prefix).unwrap(), 16)?,
        "0b" => i64::from_str_radix(input.strip_prefix(prefix).unwrap(), 2)?,
        "0o" => i64::from_str_radix(input.strip_prefix(prefix).unwrap(), 8)?,
        _ => input.parse::<i64>()?,
    };
    Ok(result)
}

/// Returns a binary representation of a number with
/// groups of 4 digits separated by a space.
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

/// Removes duplicate clusters of 4 '1's (separated by spaces)
/// from the beginning of a binary string.
fn remove_leading_ones(binary_string: String) -> String {
    if !(&binary_string).starts_with("1") {
        binary_string
    } else {
        binary_string.replace(" 1111", "").to_string()
    }
}

fn format_binary(number: i64) -> String {
    let grouped_bin = group_binary(number);
    if number < 0 {
        remove_leading_ones(grouped_bin)
    } else {
        grouped_bin
    }
}

/// Converts a number into a hex string
/// Removes duplicate leading 'F's for negative numbers.
fn format_hex(number: i64) -> String {
    if number >= 0 {
        return format!("{:X}", number);
    }
    let hex_str = format!("{:X}", number).trim_start_matches("F").to_string();
    format!("F{hex_str}")
}


fn format_octal(number: i64) -> String {
    if number >= 0 {
        return format!("{:o}", number);
    }
    let mut result_str = String::from("17[...]7");
    result_str.push_str(format!("{:o}", number)[2..].trim_start_matches("7"));
    result_str
}


fn print_num(number: i64) {
    let padding = " ".repeat(2);
    println!("Here's different representations of your number:");
    println!("Dec:{padding}  {}", number);
    println!("Hex:{padding}0x{}", format_hex(number));
    println!("Bin:{padding}0b{}", format_binary(number));
    println!("Oct:{padding}0o{}", format_octal(number));
    println!("{}", "-".repeat(20));
}
