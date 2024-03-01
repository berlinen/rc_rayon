// ![warn(rust_2018_idioms)] 这个属性会让 Rust 编译器在你的代码中使用了 Rust 2015 版本的习语，但在 Rust 2018 版本中有更好的替代时发出警告。这可以帮助你更新你的代码，使其更符合 Rust 2018 版本的习语。
#![warn(rust_2018_idioms)]

use std::{env, io, io::prelude::*, process::exit};

mod cpu_time;

fn main() {
    println!("Hello, world!");
}
