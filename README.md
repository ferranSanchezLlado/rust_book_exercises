# Rust Book Exercises

This project is a collection of solutions to the exercises and problems in 
[The Rust Programming Language](https://doc.rust-lang.org/book/), as of version [Rust](https://www.rust-lang.org/) 1.58 
(released 2022-01-13) or later. 

## Summary

The project is organized into a number of library files, each of which contains a number of modules that implement the
solutions to a particular exercise. The files are named after the chapter where the exercise is found and not the
specific subchapter. The modules contain a small description of the exercise (extracted from the book) and the code
that implements the solution. Furthermore, all modules contain a Test submodule that contains a test that checks
the solution. The project also contains a `main.rs` file for implementing the code and running some exercises that can
be run from the command line.

Most of the chapters do not have exercises, I could only find the following chapters (link to the chapter in the book):
- [common_programming_concepts_3.rs](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [common_collections_8.rs](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [generic_types_traits_and_lifetimes_10.rs](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [an_io_project_building_a_command_line_program_12.rs](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [functional_language_features_iterators_and_closures_13.rs](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- [more_about_cargo_and_crates_io_14.rs](https://doc.rust-lang.org/stable/book/ch14-00-more-about-cargo.html)
- [object_oriented_programming_features_of_rust_17.rs](https://doc.rust-lang.org/book/ch17-00-oop.html)
- [final_project_building_a_multithreaded_web_server_20](https://doc.rust-lang.org/stable/book/ch20-00-final-project-a-web-server.html)

The exercise of the chapter `More about Cargo and Crates.io` is implemented in the 
`more_about_cargo_and_crates_io_14/add` folder. This because is focused on adding a new crate to the workspace and could
not be solved in a single module. The project could be restructured to have a single crate for the exercises, but as
the project is more focused on possible solutions to the exercises, I decided to keep the previously mentioned 
structure as it allows to structure the project in a single create. Also, you can interact through the module inside the
file `more_about_cargo_and_crates_io_14.rs` where the two crates are re-imported.

## Installation

Basically: Clone the repository at the latest tag, run cargo install.

```bash
git clone https://github.com/ferranSanchezLlado/rust_book_exercises.git
cd rust_book_exercises
cargo install --force --path .
```

If there are installation errors, ensure that your toolchain is up-to-date. For the latest, run:

```bash
rustup update
```

## Usage

The project could be used as reference for the exercises. The following commands can be used to run the project:

To run the main executable, run:
```bash
cargo run
```

To run the tests, run:
```bash
cargo test
```

Especial mention to the `more_about_cargo_and_crates_io_14` chapter, which is implemented in the 
`more_about_cargo_and_crates_io_14/add` folder. So to run this chapter, you first need to go to the folder and
you can repeat the previous execution.
```bash
cd src/more_about_cargo_and_crates_io_14/add
```

## Requirements

The exercises do not require any additional libraries, except for the standard library and the two internal crates 
(which are included).

However, the tests require [paste](https://docs.rs/paste/latest/paste/) to allow the union between the identifiers of 
two functions, avoiding the need to rewrite the code for each individual implementation. The paste is only used in the
tests of the `closures` module in `functional_language_features_iterators_and_closures_13.rs`.

Furthermore, the last exercise of the `final_project_building_a_multithreaded_web_server_20` chapter asks to use an
external crate for `ThreadPool`. The crate that I used is [threadpool](https://docs.rs/threadpool/1.8.1/threadpool/), 
which could be replaced by the current implementation of the `ThreadPool` without the need to change the code.

## License

The project is licensed under a dual license: [MIT](LICENSE-MIT) and [Apache-2.0](LICENSE-APACHE).
