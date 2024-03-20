fn main() {
    let text = String::from("abc");
    println!("{}", text.len());

    let text = String::from("éáő");
    println!("{}", text.len());

    let text = String::from("שלם");
    println!("{}", text.len());

    let text = String::from("😍🎃🥽");
    println!("{}", text.len());
}

