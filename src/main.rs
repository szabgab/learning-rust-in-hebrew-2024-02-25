macro_rules! prt {
    ($var: expr) => {
        println!("{:p} {:?} {}", &$var, $var.as_ptr(), $var);
    };
}

fn main() {
    let mut text = String::from("Hello foo, how are you?");    
    prt!(text);
    let name = &text[6..9].to_owned();
    prt!(name);
    text = String::from("Hello bar, how are you?");
    prt!(text);
    prt!(name);
}

