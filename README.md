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

* [video](https://youtu.be/Z3bpIZrjvUY)
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

## 6.

