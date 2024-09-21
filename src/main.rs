use std::io::{self, Write};

struct Conversion<'a> {
    value: String,
    start_base: u32,
    target_base: Vec<&'a str>,
}

impl Conversion<'_> {
    fn convert(&self) -> Vec<String> {
        let mut numbers: Vec<String> = Vec::new();

        for (i, base) in self.target_base.clone().into_iter().enumerate() {
            if [0, 1].contains(&i) {
                continue;
            }

            let base: u32 = base.parse().expect("invalid base!");
            let decimal: u32 =
                u32::from_str_radix(&self.value, self.start_base).expect("invalid number!");
            let mut number: String = String::new();

            let remainder = decimal % base;
            let mut quotient = decimal / base;
            let mut a = String::new();
            a.push_str(&format!("{remainder}{number}"));
            number = a;

            while quotient != 0 {
                let remainder = quotient % base;
                quotient /= base;

                let mut b = String::new();
                b.push_str(&format!("{remainder}{number}"));

                number = b;
            }
            numbers.push(number)
        }
        numbers
    }
}

fn main() {
    let start_text: &str = "
┏━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ NUMERAL SYSTEM CONVERTER ┃
┣━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃ Enter \x1b[91mstart number\x1b[0m,      ┃
┃ \x1b[92mstart base\x1b[0m and           ┃
┃ \x1b[93mtarget base(s)\x1b[0m           ┃
┃ (e.g. '\x1b[91m4 \x1b[92m6 \x1b[93m16 7\x1b[0m')        ┃
┣━━━━━━━━━━━━━━━━━━━━━━━━━━┛";
    println!("{start_text}");
    print!("┃ ");
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let args: Vec<&str> = input.trim().split_whitespace().collect();
    if args.len() < 3 {
        println!("Too few arguments given!");
        return;
    }
    let (value, start_base, target_base): (String, u32, Vec<&str>) = (
        args[0].to_string(),
        args[1].parse().expect("invalid start base!"),
        args,
    );

    let a: Conversion = Conversion {
        value,
        start_base,
        target_base,
    };

    let results = a.convert();

    let mut max_length: usize = 31;

    for (i, res) in results.iter().enumerate() {
        let output: String = format!("┃ Base {}: {} ┃", a.target_base[i + 2], res);
        if output.len() > max_length {
            max_length = output.len();
        }
    }

    println!("┣{}┓", "━".repeat(max_length - 5));
    let output: String = format!("┃ {} from base {} to: ┃", a.value, a.start_base);
    println!(
        "┃ {} from base {} to: {} ┃",
        a.value,
        a.start_base,
        " ".repeat(max_length - output.len())
    );
    println!("┣{}┫", "╍".repeat(max_length - 5));

    for (i, res) in results.iter().enumerate() {
        let output: String = format!("┃ base {}: {} ┃", a.target_base[i + 2], res);
        let difference = max_length - output.len();
        println!(
            "┃ base {}: {} {} ┃",
            a.target_base[i + 2],
            res,
            " ".repeat(difference)
        );
        if results[i] != *results.last().unwrap() {
            println!("┠{}┨", "─".repeat(max_length - 5));
        }
    }
    println!("┗{}┛\n", "━".repeat(max_length - 5));
}
