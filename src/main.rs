use std::io::{self, Write};

fn main() {
    loop {
        print!("felish>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        println!("入力: {}", input);
    }
}
