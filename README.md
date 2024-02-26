# Rust


* [Rust language](https://www.rust-lang.org/)
* [Clippy documentation](https://doc.rust-lang.org/stable/clippy/usage.html)



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


## Command line arguments

* [video 4](https://youtu.be/CxtGLZLdyj8)


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


## 5 improvements

* [video ](https://youtu.be/Z3bpIZrjvUY)


* Turbofish
* `-q`
* `cargo fmt`
* [absolute_paths](https://rust-lang.github.io/rust-clippy/master/index.html#absolute_paths)
* `use` to import crates


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
