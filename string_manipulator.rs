use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a string to manipulate");
        return;
    }

    let string = &args[1];

    println!("Original string: {}", string);
    println!("Reversed string: {}", reverse_string(string));
    println!("Uppercased string: {}", string.to_uppercase());
    println!("Lowercased string: {}", string.to_lowercase());
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}
