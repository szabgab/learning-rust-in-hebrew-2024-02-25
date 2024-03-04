fn main() {
    let a = 19;
    let b = 23;
    let c = add(a, b);
    println!("{c}");
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

#[test]
fn check_add() {
    assert_eq!(42, add(19, 23));
    assert_eq!(0, add(-1, 1));
}
