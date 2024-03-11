# Rust


* [Rust language](https://www.rust-lang.org/)

## 1. Install Rust - Hello world

* [video 1](https://youtu.be/BKcMkB_R-7k)
* [page](https://he.code-maven.com/rust-course-1)

* 00:00 Environment, VirtualBox, [Install Rust](https://www.rust-lang.org/learn/get-started) on Windows (install Visual Studio as a prerequisite).
* 01:45 Install [Vistual Studio Code](https://code.visualstudio.com/).
* 02:20 I have folder `c:\work` on Windows or `~/work` on Linux and macOS.
* 03:10 Run `cmd` then `cd \work` on Windows.
* 03:50 `rustc -V` and `rustc -vV`.
* 04:35 First project in Rust `cargo new hello_world`. See the content of the folder using the file explorer.
* 06:20 Run VS Code: `code .`
* 06:40 Install the extension called **rust-analyzer** of VS Code.
* 07:22 Looking at the generated `src/main.rs`.
* 08:10 In the terminal type `cargo run`.
* 08:40 `Cargo.toml` - dependencies, edition.
* 10:55 Looking at the code: `println!` macro.
* 12:10 The `Cargo.lock` file.
* 13:00 Shall we add `Cargo.lock` to git? For programs yes, for libraries no.
* 13:30 `.gitignore` and the `target/` folder.
* 13:50 The `target/debug` folder and the `hello_world.exe`. (on Windows)
* 14:30 `\target\debug\hello_world.exe` on Windows or `./target/debug/hello_world` on Linux and macOS.
* 15:00 `cargo build` to compile the code.
* 15:55 `cargo build --release` to compile in release mode.
* 16:15 `\target\release\hello_world.exe` on Windows or `./target/release/hello_world` on Linux and macOS.

```rust
fn main() {
    println!("Hello, world!");
}
```

## 2. println and a short introduction to Clippy

* [video 2](https://youtu.be/uBcTgtbDhww)
* [page](https://he.code-maven.com/rust-course-2)

* 00:00 Plan
* 00:30 `cargo run`
* 00:40 `let` creating a variable. Strings are in double quotes.
* 01:30 Yellow underline is warning. Put the mouse on top to see the actual message. Prefix the variable with `_` if we don't intend to use it any more.
* 02:20 Curly braces `{}` as a placeholder in a string.
* 03:20 Clippy - linter `cargo clippy` - [Clippy source](https://github.com/rust-lang/rust-clippy) with some of the lint groups.
* 04:50 `cargo clippy -- -D clippy::pedantic`
* 06:45 [uninlined_format_args](https://rust-lang.github.io/rust-clippy/master/index.html#/uninlined_format_args)
* 07:40 `Cargo.toml` to configure Clippy (see below)
* 08:20 Get help about Clippy `cargo clippy -- --help`
* 08:50 I made typos in the Clippy configuration.
* 09:50 `cargo clippy --fix` to fix the code as Clippy recommends.
* 11:00 `git commit` using VS Code buttons.
* 12:15 Run `cargo clippy --fix`.



```rust
fn main() {
    let name = "Foo Bar";
    println!("Hello, {name}!");
}
```

The **Clippy** configuration in `Cargo.toml`:

```toml
[lints.clippy]
pedantic = "deny"
```


## 3. Convert string to number, error handling

* [video 3](https://youtu.be/CN7mOZMLZs0)
* [page](https://he.code-maven.com/rust-course-3)

* 00:00 We switched to Linux
* 01:20 README - [Clippy documentation](https://doc.rust-lang.org/stable/clippy/usage.html)
* 02:30 Error handling in Rust
* 03:05 Error handling in Python, Java with exceptions in Perl and C with return -values.
* 04:00 Input from the user or from files is always a string. Even when it looks like a number.
* 05:00 `input.parse()`
* 06:40 `i32`  `i8`
* 07:45 What is this [Result](https://doc.rust-lang.org/std/result/enum.Result.html)?  `enum`.
* 09:00 [unwrap](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap) will panic!.
* 10:10 Comment.
* 10:15 `dbg!` macro.
* 10:50 Seing the first panic.
* 11:35 `match`, `Ok()`, `Err()`.
* 14:00 Explain `match` and the two arms.
* 16:00 `eprintln!` and `std::process::exit`.


```rust
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
}
```


## 4. Command line arguments

* [video 4](https://youtu.be/CxtGLZLdyj8)
* [page](https://he.code-maven.com/rust-course-4)

* 00:00 Plan
* 01:00 Get parameters from the command line `std::env::args()`.
* 02:00 The [Display trait](https://doc.rust-lang.org/std/fmt/trait.Display.html) is missing. Use `dbg!(args)`.
* 02:50 I changed the name of the folder including the date of the course.
* 03:55 The name of the executable by default is the name of the project. (The crate).
* 04:50 Change the placeholder to be: `{:?}` (debug printing).
* 06:10 How to get the individual parameters from the command line?
* 06:20 `collect()`
* 07:00 Define the type of what we expect from `collec()`.
* 08:20 print the vector
* 08:30 Access the length of the vector using `len()`.
* 09:10 Access an element of the vector.
* 09:35 Change the name of the package, and thus the name of the generated executable. `Cargo.toml`.
* 11:00 Check if the number of parametes is as expected, exit with usage message if not.
* 12:20 What is the name of the program?
* 15:55 What does clippy say here?
* 16:10 Statement inside the curly braces in a string does not work.


```rust
fn main() {
    // let args = std::env::args();
    // //dbg!(args);
    // println!("{:?}", args);

    let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);
    // println!("{args:?}");
    // println!("{}", args.len());
    // println!("{}", args[1]);
    if args.len() != 3 {
        eprintln!("Usage: {} NUMBER NUMBER", args[0]);
        std::process::exit(1);
    }

    let number_1: i32 = args[1].parse().unwrap();
    let number_2: i32 = args[2].parse().unwrap();
    println!("{}", number_1 + number_2);
}
```


## 5. 4-5 improvements

* [video 5](https://youtu.be/Z3bpIZrjvUY)
* [page](https://he.code-maven.com/rust-course-5)

* 00:00 Review, clean commented code.
* 00:30 Using [Turbofish](https://turbo.fish/) `::<>`. The editor showing the type.
* 03:50 Quiet mode: `cargo run -q`.
* 04:30 Code beautifier (prettifier, code formatter) `cargo fmt`
* 05:40 Full (or absolute) path of functions.
* 06:45 Adding lint called [absolute_paths](https://rust-lang.github.io/rust-clippy/master/index.html#absolute_paths).
* 07:55 `use` to import packages, function, or traits.


```rust
use std::env;
use std::process::exit;

fn main() {
    //let args = std::env::args().collect::<Vec<String>>();  // Turbofish ::<>
    let args = env::args().collect::<Vec<String>>(); // Turbofish ::<>

    if args.len() != 3 {
        eprintln!("Usage: {} NUMBER NUMBER", args[0]);
        //std::process::exit(1);
        exit(1);
    }

    let number_1 = args[1].parse::<i32>().unwrap();
    let number_2 = args[2].parse::<i32>().unwrap();
    println!("{}", number_1 + number_2);
}
```

## 6. Numbers and types of numbers

* [video 6](https://youtu.be/Dkq-C4zWZyo)
* [page](https://he.code-maven.com/rust-course-6)

* 00:00 Preparation.
* 00:47 Start computing the area of rectangle.
* 01:17 default type `i32`.
* 02:29 In VS Code Press `F8` to get next error.
* 03:05 `cargo clippy --fix --allow-dirty`.
* 04:30 Setting the type to `i16`. Deducting the type from the mathematical operation.
* 06:30 Multiply `i32` by `i16`.
* 07:00 Deduct the type from the operation.
* 07:50 Different ways to set the type.
* 09:40 What numerical types are in Rust? `i8` .. `i128`, `u8` .. `u128`, `f32`, `f64`.
* 10:50 range `..`
* 12:43 floating point.
* 13:30 Using `i8`, overflow - compilation error.
* 15:27 Getting numbers from the command line.
* 18:22 Rust could not guess the type of the second parameter.
* 19:50 panic if the result of a numerical operation does not fit in the variable.
* 20:35 `cargo build --release` Compile in release mode.
* 20:50 `cargo run --release` Run in release mode. - overflow (but no panic!).
* 21:50 In debug mode overflow panics, in release mode it just overflows.
* 22:40 Method that has `mul` in it, 4 methods to multiply. Using `saturating_mul`.


```rust
fn main() {
    let width = 11;
    let length = 12;
    let area = width * length;
    println!("{area}");
}
```

```rust
fn main() {
    let width: i16 = 11;
    //let width = 11i16;
    //let width = 11_i16;
    //let width = 11 as i16;
    let length = 12;
    let area = width * length;
    println!("{area}");
}
```

```rust
fn main() {
    // i8..i128
    // u8..u128
    // f32, f64
    // invalid:
    let z: i8 = 130;
    //let z: u8 = 256;
    //let z = 3.14;
}
```

```rust
fn main() {
    // compile error, but not in the editor
    let width = 11_i8;
    let length = 12;
    let area = width * length;
    println!("{area}");
}
```


```rust
use std::env;
use std::process::exit;

fn main() {

    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} WIDTH LENGTH", args[0]);
        exit(1);
    }

    // let width = args[1].parse::<i8>().unwrap();
    // // This does not work
    // // let length = args[2].parse().unwrap();
    // let length = args[2].parse::<i8>().unwrap();
    // let area = width * length;
    // println!("{area}");

    let width = args[1].parse::<i8>().unwrap();
    // This does not work
    // let length = args[2].parse().unwrap();
    let length = args[2].parse::<i8>().unwrap();
    let area = width.saturating_mul(length);
    println!("{area}");

}
```


## 7. Function and testing

* [video 7 ](https://youtu.be/kP20cF-TItA)
* [page](https://he.code-maven.com/rust-course-7)

* 00:00 Defining a function, parameters, return value.
* 01:50 Using the function.
* 02:40 Running the code.
* 02:55 Running tests using `cargo test`.
* 03:28 Writing a unitest.
* 04:00 `assert_eq!`.
* 04:23 `#[test]`.
* 04:40 Running tests - success.
* 05:10 Running tests - failure.
* 05:55 Testing the real function.

```rust
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
```


## 8. Count instances in an array

* [video 8](https://youtu.be/5XiPCIFX_n0)
* [page](https://he.code-maven.com/rust-course-8)

* 00:00 Start
* 00:20 Define a vector of numbers using `vec![]`.
* 01:40 `println!` with `{}`, `{:?}`, or with `{:#?}` to pretty print.
* 02:40 `for` loop on the elements of a vector.
* 03:30 Loop on the indexes. Range using `..` and `len()`.
* 05:54 `cargo clippy`: Switch from vector to array.
* 07:30 Count instances.
* 08:10 Marking a variable as mutable using `mut`.
* 09:20 `cargo clippy`: use the `+=` operator.
* 11:15 Save currnt version in README, Move code to function.
* 12:30 Refactor: Mark code, right-click on the mouse, click on "Refactor" and "Extract into function".
* 13:00 The parameters of this new function.
* 14:35 Run the code after refactoring.
* 14:45 Rename function (`F2`).
* 15:30 Commit the files to `git`.
* 16:00 Writing a test `#[test]`.
* 17:15 `cargo test`.
* 18:00 Try passing an array of a different length. **mismatched types**.

```rust
fn main() {
    let numbers = [23, 8, 4, 7, 7, 4, 19];
    //println!("{numbers:?}");
    // for ix in 0..numbers.len() {
    //     println!("{ix} {}", numbers[ix]);
    // }
    let number = 4;
    let mut counter = 0;
    for value in numbers {
        //println!("{value}");
        if value == number {
            //counter = counter + 1;
            counter += 1;
        }
    }
    println!("Number of time {number} appears in {numbers:?} is {counter}");
}
```

```rust
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
```

## 9. Make function more generic - arbitrary length of array and arbitrary type of values.

* [video 9](https://www.youtube.com/watch?v=eQ1gruzOvL4)
* [page](https://he.code-maven.com/rust-course-9)

* 00:00 Recap of the result from the previous video and the problem we have.
* 00:38 The current function signature.
* 01:25 Replace `[i32; 7]` by `&[i32]` in he function signature.
* 04:20 `cargo run` already works.
* 04:30 Fixing the tests.
* 05:25 The type is not generic enough.
* 05:57 `cargo clippy` is happy
* 06:25 Save the code in the README
* 07:25 Accept any types of values.
* 11:14 Test counting instances of a string.
* 12:18 `cargo clippy`
* 13:00 It is enough to pass the parameter by reference.
* 14:40 Rename the function and he parameters.
* 15:18 `F2` to rename the function and the parameters.

```rust
fn main() {
    let numbers = [23, 8, 4, 7, 7, 4, 19];
    let number = 4;

    let counter = count_number(&numbers, number);
    println!("Number of time {number} appears in {numbers:?} is {counter}");
}

fn count_number(numbers: &[i32], number: i32) -> i32 {
    let mut counter = 0;
    for value in numbers {
        //println!("{value}");
        if value == &number {
            //counter = counter + 1;
            counter += 1;
        }
    }
    counter
}

#[test]
fn check_counter() {
    assert_eq!(count_number(&[23, 8, 4, 7, 7, 4, 19], 4), 2);
    assert_eq!(count_number(&[23, 8, 4, 7, 7, 4, 19], 23), 1);
    assert_eq!(count_number(&[23, 8, 4, 7, 7, 4, 19], 9), 0);
    assert_eq!(count_number(&[23, 8, 4], 23), 1);
}
```


```rust
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
```


## 10. Get the values from the command line

* [video 10](https://youtu.be/fMfs1Rxarhc)
* [page](https://he.code-maven.com/rust-course-10)

* 00:00 Review and plan.
* 00:30 Command line example.
* 01:00 `std::env::args().collect::<Vec<String>>()`.
* 03:10 Add TODO checking number of parameters.
* 03:30 Fixing the compiler issue adding `&` in-front of the `args`.
* 05:15 fixing issues reported by `cargo clippy`: Full path and unnecessary use of `&` when passing argument.
* 06:55 Two extra things: the TODO and the `use` statements.
* 07:05 Merge the `use` statements.
* 07:30 What happens if the user does not provide any parameters?


```rust
use std::{cmp, env, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} VALUE VALUEs", args[0]);
        process::exit(1);
    }
    let value = &args[1];
    let values = &args[2..];

    let counter = count_instances(values, value);
    println!("Number of time {value} appears in {values:?} is {counter}");
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
```

## 11. Read values from file, disallow unwrap and expect - use match


* [video 11]()
* [page](https://he.code-maven.com/rust-course-11)

* 00:00 Review and plan.


`Cargo.toml`

```toml
[package]
name = "demo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]


[lints.clippy]
pedantic = "deny"
absolute_paths = "deny"
# unwrap_used = "warn"
unwrap_used = "deny"
expect_used = "deny"
```

`src/main.rs`

```rust
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
```

