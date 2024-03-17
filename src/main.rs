fn main() {
    let mut text = String::new();
    println!("{}", text.len());

    text.push_str("Hello");
    println!("{}", text.len());

    // text.push_str("éáöüí");
    // println!("{}", text.len());

    // text.push_str("ש");
    // println!("{}", text.len());

    text.push_str("😂😝🤣");
    println!("{}", text.len());
    println!();

    //text[2]

    // for ch in text.chars() {
    //     println!("{ch}");
    // }
    // println!();


    // for byte in text.bytes() {
    //     println!("{byte}");
    // }
    // println!();

    text = String::from("hello");
 
    //let sub = &text[0..3];
    let sub = &text[0..3].to_owned();
    println!("{:?}", text.as_ptr());
    println!("{:?}", sub.as_ptr());
    text = String::from("hello");
    println!("{}", text);
    println!("{}", sub);


}

