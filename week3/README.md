# Rust course - Week 3 exercise

This week's exercise will introduce enums, pattern matching, if let, teach you about bringing in crates and introduce how unit testing is done in Rust.

Ensure your editor is set to format on save, and that you have the `rustfmt` tool installed. You can install it by running `rustup component add rustfmt`.

### Exercise: Enums

Enums are one of Rust's most powerful features. They allow us to create a type with a finite set of values.

Create an `enum` called `Shape` with three variants: `Circle`, `Square` and `Rectangle`, and give each variant appropriate fields

<details><summary>SPOILER</summary>

- `Circle { radius }`
- `Square { length }`
- `Rectangle { width, height }`

</details>

### Exercise: Pattern matching

Create a function named `area` that takes a `Shape` reference as an argument and returns the area of the shape.

Use pattern matching to determine what area to return for each of the variants.

### Exercise: Unit testing

Create a unit test named `test_shape_areas` (yes, in the same file) that tests whether your `area` function is correctly calculating the area of each of the shapes.
Use the `assert_eq!` macro to test the area of each shape.
Use `#[test]` to mark the function as a test, and `cargo test` to run the tests.

### Exercise: Crate

Bring in the `rand` crate to your project and create a function named `random_shape` that returns a randomly generated `Shape`.
You can install crates by either adding them to your `Cargo.toml` file, or by running `cargo add <crate_name>`.

Make sure to read the documentation for the `rand` crate to figure out how to generate a random number.

How do you convert a random number to a Shape? Use pattern matching with numbers!

### Exercise: if let

Create a function named `circle_area` that takes a `Shape` reference as an argument and returns the area of the shape _if_ it is a Circle, otherwise it should return 0. The return type should be `f64`.

Use an `if let` statement to determine whether the shape is a Circle and return the area if it is.

Write a test that verifies that this function works correctly.

### Exercise: Option

Change the `circle_area` function to return an `Option<f64>` instead of an `f64`. If the shape is a Circle, return `Some(area)`, otherwise return `None`.

Update the test accordingly.
