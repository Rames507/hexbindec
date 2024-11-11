pub fn print_help_msg() -> () {
    println!(
        "Enter a number with its corresponding prefix to display it. Omit prefix for decimal numbers.\n\
        Available prefixes are '0b' for binary, '0x' for hexadecimal and '0b' for Octal.\n\
        Spaces in the input will be ignored.\n\
        Valid inputs include:\n\
        {tab}0b01101010\n\
        {tab}0b0110 1010\n\
        {tab}0x37F20A\n\
        {tab}42",
        tab = "    "
    );
    println!(r#"Type "c", "clear" or "cls" to clear the terminal window."#)
}
