fn main() {
    let text = String::from("abcéáő😍🎃🥽");
    // println!("{}", text.len());
    // println!("{}", &text[0..=12]);

    // for byte in text.bytes() {
    //     println!("{byte}");
    // }

    // for ch in text.chars() {
    //     println!("{ch}");
    // }

    //println!("{}", text.chars().nth(7).unwrap());
    //println!("{}", text.chars().nth(11).unwrap());

    let index = 11;
    match text.chars().nth(index) {
        Some(val) => println!("{val}"),
        None => println!("does not exist"),
    }

    // println!("{}", match text.chars().nth(index) {
    //     Some(val) => val,
    //     None => "does not exist",
    // });

}

