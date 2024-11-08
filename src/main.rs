use std::io;
use std::num::ParseIntError;

fn main() {
    loop {
        let user_input = get_user_input();
        if ["c", "clear", "cls"].contains(&user_input.as_str()) {
            print!("{}c", 27 as char); // clears the terminal screen
            continue;
        }
        let number = match parse_user_input(&user_input) {
            Ok(num) => num,
            Err(e) => {
                println!("Failed to parse the number: {e}");
                continue;
            }
        };
        print_num(number);
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    println!("Please enter a number (0x/0b/0o):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().replace(" ", "").to_string()
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

/// Groups an input string into batches of `group_size` characters.
///
/// # Arguments
///
/// * `ungrouped`: The string to be split into groups
/// * `group_size`: size of individual groups
/// * `separator`: The separator that is inserted between the groups
/// * `padding_char`: if a padding character is passed it will be used to fill the first group
///    in case the input isn't divisible by `group_size`
///
/// returns: String
///
/// # Examples
///
/// ```
/// assert_eq!("0010 1011", group_into("101011", 4, " ", Some('0')));
/// ```
pub fn group_into(
    ungrouped: String,
    group_size: usize,
    separator: &str,
    padding_char: Option<char>,
) -> String {
    // this is needed because even if you don't want padding, it prevents the groups from shifting
    // (being aligned to the front of the string instead of the back)
    const TEMP_PAD: char = '#';

    let pad_size = (group_size - (ungrouped.len() % group_size)) % group_size;
    let padding = padding_char
        .unwrap_or(TEMP_PAD)
        .to_string()
        .repeat(pad_size);
    let padded_string = padding + &ungrouped;

    let chunks: Vec<String> = padded_string
        .as_bytes()
        .chunks(group_size)
        .map(|chunk| std::str::from_utf8(chunk).unwrap().to_string())
        .collect();

    let grouped_string = chunks.join(separator);

    if padding_char.is_none() {
        grouped_string.trim_start_matches(TEMP_PAD).to_string()
    } else {
        grouped_string
    }
}

/// Converts a number to its binary representation grouped into clusters of 4 digits.
/// Removes duplicate clusters of 4 '1's (separated by spaces)
/// from the beginning of a binary string.
fn format_binary(number: i64) -> String {
    let grouped_bin = group_into(format!("{:b}", number), 4, " ", Some('0'));
    if number >= 0 {
        return grouped_bin;
    }
    grouped_bin.replace(" 1111", "").to_string()
}

/// Converts a number into a hex string.
/// Removes duplicate leading 'F's for negative numbers.
fn format_hex(number: i64) -> String {
    if number >= 0 {
        return group_into(format!("{:X}", number), 4, " ", Some('0'));
    }
    let hex_str = format!("{:X}", number).trim_start_matches("F").to_string();
    group_into(format!("F{hex_str}"), 4, " ", Some('0'))
}

/// Converts a number into an octal string.
/// Removes duplicate leading '7's for negative numbers.
fn format_octal(number: i64) -> String {
    if number >= 0 {
        return format!("{:o}", number);
    }
    let mut result_str = String::from("17[7]7");
    result_str.push_str(format!("{:o}", number)[2..].trim_start_matches("7"));
    result_str
}

fn format_decimal(number: i64) -> String {
    if !(number.abs() >= 1000) {
        return number.to_string();
    }
    group_into(number.to_string(), 3, " ", None)
}

fn print_num(number: i64) {
    let padding = " ".repeat(2);
    println!("Here's different representations of your number:");
    println!("Dec:{padding}  {}", format_decimal(number));
    println!("Hex:{padding}0x{}", format_hex(number));
    println!("Bin:{padding}0b{}", format_binary(number));
    println!("Oct:{padding}0o{}", format_octal(number));
    println!("{}", "-".repeat(20));
}
