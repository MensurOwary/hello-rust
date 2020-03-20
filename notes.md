# Notes

Cargo is your best friend.

### To create a new application and running it

Creates a new project in Rust

`cargo new project_name`

Inside the folder, doing this runs the `main.rs` file

`cargo run`

### cargo.toml file

`cargo.toml` file is the configuration of the Rust project. The simple one looks like this:

```toml
[package]
name = "project_name"
version = "0.1.0"
authors = ["Mensur Owary <mensurowary@gmail.com>"]
edition = "2018"
[dependencies]
num = "0.2"
```

`[package]` section contains information related to the project. Cargo needs those infos to compile the application. `edition` is the Rust edition. `[dependencies]` section contains the `crate`s which are Rust libraries. The executable crates are called *binary crates*, while others are *library crates*.

### A couple of Cargo commands

| Command        | It does                                  | Extra info                                                                     |
| -------------- | ---------------------------------------- | ------------------------------------------------------------------------------ |
| `cargo check`  | checks if the code compiles              |                                                                                |
| `cargo build`  | builds the project                       | `--release` flag builds the executable for release, applies some optimizations |
| `cargo run`    | builds and runs the project              |                                                                                |
| `cargo clean`  | cleans the target directory              |                                                                                |
| `cargo update` | updates the registry                     | it ignores the versions in `Cargo.lock` file and updates the crates            |
| `cargo doc`    | generates docs based on the dependencies | `--open` flag opens the docs in the browser                                    |

 
## Rusty facts

- associated functions are basically **static** methods (e.g `String::new`) and they are pertinent to the type not to an instance.