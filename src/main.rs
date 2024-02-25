use std::env;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<String>>(); // Turbofish ::<>
    if args.len() != 3 {
        eprintln!("Usage: {} NUMBER NUMBER", args[0]);
        exit(1);
    }

    let number_1 = args[1].parse::<i32>().unwrap();
    let number_2 = args[2].parse::<i32>().unwrap();
    println!("{}", number_1 + number_2);
}
