macro_rules! prt {
    ($var: expr) => {
        println!("{:2} {:2} {:p} {:<15?} '{}'", $var.len(), $var.capacity(), &$var, $var.as_ptr(), $var);
    };
}

macro_rules! prt_str {
    ($var: expr) => {
        println!("{:2}    {:p} {:<15?} '{}'", $var.len(), &$var, $var.as_ptr(), $var);
    };
}

fn main() {
    let text = "Hello foo, how are you";
    prt_str!(text);

    //let text = String::from(text);
    //let text = text.to_string();
    let text = text.to_owned();
    prt!(text);

    let name = &text[6..=8];
    prt_str!(name);

    let fname = String::from("bar");
    prt!(fname);

    let content = std::fs::read_to_string("names.txt").unwrap();
    prt!(content);
    let words = content.split('\n').collect::<Vec<&str>>();
    //prt!(words);
    prt_str!(words[0]);
    prt_str!(words[1]);
    prt_str!(words[2]);
    prt_str!(words[3]);
    println!("{:2} {:2} {:p} {:<15?}", words.len(), words.capacity(), &words, words.as_ptr());


}

