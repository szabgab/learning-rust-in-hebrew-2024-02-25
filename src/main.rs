use std::cmp;

fn main() {
    let numbers = [23, 8, 4, 7, 7, 4, 19];
    let number = 4;

    let counter = count_instances(&numbers, &number);
    println!("Number of time {number} appears in {numbers:?} is {counter}");
}

fn count_instances<T: cmp::PartialEq>(values: &[T], value_to_find: &T) -> i32 {
    let mut counter = 0;
    for value in values {
        //println!("{value}");
        if value == value_to_find {
            //counter = counter + 1;
            counter += 1;
        }
    }
    counter
}

#[test]
fn check_counter() {
    assert_eq!(count_instances(&[23, 8, 4, 7, 7, 4, 19], &4), 2);
    assert_eq!(count_instances(&[23, 8, 4, 7, 7, 4, 19], &23), 1);
    assert_eq!(count_instances(&[23, 8, 4, 7, 7, 4, 19], &9), 0);
    assert_eq!(count_instances(&[23, 8, 4], &23), 1);
    assert_eq!(count_instances(&["foo", "bar", "foo"], &"foo"), 2);

}
