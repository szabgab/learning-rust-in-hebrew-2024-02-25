mod macros;
//use macros::{prt, prt_str};

fn main() {
}

// "12341"

fn count_digits(text: &str) -> [u32; 10] {
    let mut count: [u32; 10] = [0; 10];
    for ch in text.chars() {
        let ix = ch as usize - '0' as usize;
        count[ix] += 1;
    }

    count 
}

#[test]
fn test_empty() {
    assert_eq!(count_digits(""), [0; 10])
}

#[test]
fn test_short() {
    assert_eq!(count_digits("1"), [0, 1, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(count_digits("131"), [0, 2, 0, 1, 0, 0, 0, 0, 0, 0]);
    //assert_eq!(count_digits("a"), [0, 2, 0, 1, 0, 0, 0, 0, 0, 0]);
}