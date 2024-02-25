fn main() {
    let input = "420x";
    //let number: i32 = input.parse().unwrap();

    let number: i32 = match input.parse() {
        Ok(val) => val,
        //Err(err) => panic!("not good {err}"),
        Err(err) => {
            eprintln!("not good {err}");
            std::process::exit(1);
        },
    };

    dbg!(number);
    // let name = "Foo Bar";
    // println!("Hello, {name}!");
}
