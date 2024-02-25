# Rust


* [Rust language](https://www.rust-lang.org/)
* [Clippy documentation](https://doc.rust-lang.org/stable/clippy/usage.html)
* [Clippy source](https://github.com/rust-lang/rust-clippy)


## Hello world

* [video 1](https://youtu.be/BKcMkB_R-7k)
* [video 2](https://youtu.be/uBcTgtbDhww)

```rust
fn main() {
    let name = "Foo Bar";
    println!("Hello, {name}!");
}
```

```
cargo new hello_world
cargo run
cargo build
cargo build --release

cargo clippy
cargo clippy -- -D clippy::pedantic
cargo clippy --fix
```

## Convert string to number, error handling

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
