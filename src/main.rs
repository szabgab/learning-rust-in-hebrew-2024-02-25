macro_rules! prt {
    ($var: expr) => {
        println!("{:2} {:2} {:p} {:<15?} '{}'", $var.len(), $var.capacity(), &$var, $var.as_ptr(), $var);
    };
}

// ptr
// len
// capacity
fn main() {
    //let mut text = String::new();    
    let mut text = String::with_capacity(50);    
    prt!(text);

    text.push('a');
    prt!(text);

    let name = String::from("foo");
    prt!(name);

    text.push('b');
    prt!(text);

    text.push_str("123456");
    prt!(text);

    text.push('x');
    prt!(text);

    text.push_str("123456789012345678901234567890");
    prt!(text);


}

