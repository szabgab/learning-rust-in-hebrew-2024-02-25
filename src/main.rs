fn main() {
    let numbers = [23, 8, 4, 7, 7, 4, 19];
    let number = 4;

    let counter = count_number(numbers, number);
    println!("Number of time {number} appears in {numbers:?} is {counter}");
}

fn count_number(numbers: [i32; 7], number: i32) -> i32 {
    let mut counter = 0;
    for value in numbers {
        //println!("{value}");
        if value == number {
            //counter = counter + 1;
            counter += 1;
        }
    }
    counter
}

#[test]
fn check_counter() {
    assert_eq!(count_number([23, 8, 4, 7, 7, 4, 19], 4), 2);
    assert_eq!(count_number([23, 8, 4, 7, 7, 4, 19], 23), 1);
    assert_eq!(count_number([23, 8, 4, 7, 7, 4, 19], 9), 0);
    //assert_eq!(count_number([23, 8, 4], 23), 1);
}