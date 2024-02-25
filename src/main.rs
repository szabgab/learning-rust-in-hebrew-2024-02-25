fn main() {
    // let args = std::env::args();
    // //dbg!(args);
    // println!("{:?}", args);

    let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);
    // println!("{args:?}");
    // println!("{}", args.len());
    // println!("{}", args[1]);
    if args.len() != 3 {
        eprintln!("Usage: {} NUMBER NUMBER", args[0]);
        std::process::exit(1);
    }

    let number_1: i32 = args[1].parse().unwrap();
    let number_2: i32 = args[2].parse().unwrap();
    println!("{}", number_1 + number_2);
}
