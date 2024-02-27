use std::env;
use std::process::exit;

fn main() {
    // let width = 11;
    // let length = 12;
    // let area = width * length;
    // println!("{area}");

    //let width: i16 = 11;
    //let width = 11i16;
    // let width = 11_i16;
    // //let width = 11 as i16;
    // let length = 12;
    // let area = width * length;
    // println!("{area}");

    // i8..i128
    // u8..u128
    // f32, f64
    // invalid:
    //let z: i8 = 130;
    //let z: u8 = 256;
    //let z = 3.14;

    // compile error, but not in the editor
    // let width = 11_i8;
    // let length = 12;
    // let area = width * length;
    // println!("{area}");

    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} WIDTH LENGTH", args[0]);
        exit(1);
    }

    // let width = args[1].parse::<i8>().unwrap();
    // // This does not work
    // // let length = args[2].parse().unwrap();
    // let length = args[2].parse::<i8>().unwrap();
    // let area = width * length;
    // println!("{area}");

    let width = args[1].parse::<i8>().unwrap();
    // This does not work
    // let length = args[2].parse().unwrap();
    let length = args[2].parse::<i8>().unwrap();
    let area = width.saturating_mul(length);
    println!("{area}");

}
