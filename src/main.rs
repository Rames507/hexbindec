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
    println!("Please enter a number (0x/0b/0 ):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();

    parser_user_input(&input)
}

fn parser_user_input(input: &String) -> Result<i64, ParseIntError> {
    let result = match &input[0..2] {
        "0x" => i64::from_str_radix(input.strip_prefix("0x").unwrap(), 16)?,
        "0b" => i64::from_str_radix(input.strip_prefix("0b").unwrap(), 2)?,
        "0 " => i64::from_str_radix(input.strip_prefix("0 ").unwrap(), 8)?,
        _ => input.parse::<i64>()?,
    };
    Ok(result)
}

fn print_num(number: i64) {
    println!("Your number in different bases:");
    println!("Dec:\t  {}", number);
    println!("Hex:\t0x{:X}", number);
    println!("Bin:\t0b{:b}", number);
    println!("Oct:\t0 {:o}", number);
    println!("{}", "-".repeat(20));
}
