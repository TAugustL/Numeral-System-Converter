use std::io;

struct Conversion {
    start_type: String,
    start_value: String,
}

impl Conversion {
    fn calc_target_value(&self) {
        if self.start_type == "dec" {
            let decimal: u128 = u128::from_str_radix(&self.start_value, 10).unwrap();

            let dec_line: usize = format!("|Results for {}: |", decimal).len();
            let hex_line: usize = format!("|Hex number: 0x{:x} |", decimal).len();
            let bin_line: usize = format!("|Bin number: 0b{:b} |", decimal).len();
            let oct_line: usize = format!("|Oct number: 0o{:o} |", decimal).len();

            println!("\n{}", "—".repeat(bin_line));
            println!("|Results for {}:{} |", self.start_value, " ".repeat(bin_line - dec_line));
            println!("|{}|", " ".repeat(bin_line - 2));
            println!("|Hex number: {}{} |", format!("0x{:x}", decimal), " ".repeat(bin_line - hex_line));
            println!("|Bin number: {} |", format!("0b{:b}", decimal));
            println!("|Oct number: {}{} |", format!("0o{:o}", decimal), " ".repeat(bin_line - oct_line));
            println!("{}", "—".repeat(bin_line));

        } else if self.start_type == "bin" {
            let decimal: u128 = u128::from_str_radix(&self.start_value, 2).unwrap();

            let dec_line: usize = format!("|Dec number: {} |", decimal).len();
            let hex_line: usize = format!("|Hex number: 0x{:x} |", decimal).len();
            let bin_line: usize = format!("|Results for 0b{:b}: |", decimal).len();
            let oct_line: usize = format!("|Oct number: 0o{:o} |", decimal).len();

            println!("\n{}", "—".repeat(bin_line));
            println!("|Results for 0b{}: |", self.start_value);
            println!("|{}|", " ".repeat(bin_line - 2));
            println!("|Hex number: {}{} |", format!("0x{:x}", decimal), " ".repeat(bin_line - hex_line));
            println!("|Dec number: {}{} |", decimal, " ".repeat(bin_line - dec_line));
            println!("|Oct number: {}{} |", format!("0o{:o}", decimal), " ".repeat(bin_line - oct_line));
            println!("{}", "—".repeat(bin_line));

        } else if self.start_type == "hex" {
            let decimal: u128 = u128::from_str_radix(&self.start_value, 16).unwrap();

            let dec_line: usize = format!("|Dec number: {} |", decimal).len();
            let hex_line: usize = format!("|Results for 0x{:x}: |", decimal).len();
            let bin_line: usize = format!("|Bin number: 0b{:b} |", decimal).len();
            let oct_line: usize = format!("|Oct number: 0o{:o} |", decimal).len();

            println!("\n{}", "—".repeat(bin_line));
            println!("|Results for 0x{}:{} |", self.start_value, " ".repeat(bin_line - hex_line));
            println!("|{}|", " ".repeat(bin_line - 2));
            println!("|Dec number: {}{} |", decimal, " ".repeat(bin_line - dec_line));
            println!("|Bin number: {} |", format!("0b{:b}", decimal));
            println!("|Oct number: {}{} |", format!("0o{:o}", decimal), " ".repeat(bin_line - oct_line));
            println!("{}", "—".repeat(bin_line));

        } else if self.start_type == "oct" {
            let decimal: u128 = u128::from_str_radix(&self.start_value, 8).unwrap();

            let dec_line: usize = format!("|Dec number: {} |", decimal).len();
            let hex_line: usize = format!("|Hex number: 0x{:x} |", decimal).len();
            let bin_line: usize = format!("|Bin number: 0b{:b} |", decimal).len();
            let oct_line: usize = format!("|Results for 0o{:o}: |", decimal).len();

            println!("\n{}", "—".repeat(bin_line));
            println!("|Results for 0o{}:{} |", self.start_value, " ".repeat(bin_line - oct_line));
            println!("|{}|", " ".repeat(bin_line - 2));
            println!("|Hex number: {}{} |", format!("0x{:x}", decimal), " ".repeat(bin_line - hex_line));
            println!("|Bin number: {} |", format!("0b{:b}", decimal));
            println!("|Dec number: {}{} |",  decimal, " ".repeat(bin_line - dec_line));
            println!("{}", "—".repeat(bin_line));

        } else {
            println!("Invalid input!");
        }
    }
}

fn main() {
    println!("{}", "=".repeat(24));
    println!("NUMERAL SYSTEM CONVERTER");
    println!("{}", "=".repeat(24));
    println!("Enter start number type:\n[dec] for decimal\n[hex] for hexadecimal\n[bin] for binary\n[oct] for octal\n");

    let mut num_type: String = String::new();
    io::stdin().read_line(&mut num_type).expect("Failed to read line!");
    let num_type: String = num_type.trim().parse().expect("Failed to process input!");

    let start_type: String = String::from(num_type);

    println!("\nEnter start value:\n");
    let mut num_value: String = String::new();
    io::stdin().read_line(&mut num_value).expect("Failed to read line!");
    let num_value: String = num_value.trim().parse().expect("Failed to process input!");

    let start_value: String = String::from(num_value);

    let conversion = Conversion {
        start_type,
        start_value
    };

    conversion.calc_target_value();
}
