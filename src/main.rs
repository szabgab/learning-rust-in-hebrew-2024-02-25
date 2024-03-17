use std::{cmp, env, fs, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Usage: {} VALUE FILENAME", args[0]);
        process::exit(1);
    }
    let value = &args[1];
    let filename = &args[2];
    // let content = fs::read_to_string(filename).unwrap();
    // let content = fs::read_to_string(filename).expect("Could not read file");
    let content = match fs::read_to_string(filename) {
        Ok(cont) => cont,
        Err(err) => {
            eprintln!("Error: {err} while trying to read '{filename}'");
            process::exit(2);
        }
    };

    //println!("{content}");
    let values = content.split('\n').collect::<Vec<&str>>();
    //println!("{values:?}");
    let counter = count_instances(&values, &value.as_str());
    println!("Number of time {value} appears in {values:?} is {counter}");
}

fn count_instances<T: cmp::PartialEq>(values: &[T], value_to_find: &T) -> usize {
    let counter = values
        .iter()
        .filter(|value| value == &value_to_find)
        .collect::<Vec<&T>>()
        .len();
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
