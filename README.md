# Rust course - Week 2 exercise

A simple program has been supplied. It is not finished.
Try running this program using `cargo run`. You will notice that it panics - there is an unimplemented method!

## Task 1

Implement the unfinished function `area`. Run the program again to see if it works.

## Task 2

Change the `rect1` variable to be mutable. After printing the area the first time, change the width of the rectangle to 100 and print the area again. What happens? See if you can fix it.

<details><summary>Hint</summary>You can use references to fix this. See the [Rust book](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) for more information.</details>

## Task 3

The `area` function is a plain function, but it only has 1 parameter, which has the Rectangle type.
This is a good candidate for a method. Change the function to a method, and call it using the dot notation: `rect1.area()`.

## Task 4

Add a second method to the Rectangle struct, called `can_hold`. This method should take another Rectangle as a parameter, and return a boolean. It should return true if the rectangle it is called on can hold the rectangle passed as a parameter, and false if it cannot.

Now create a new rectangle instance in your main method, and pass it to the `can_hold` method called on the first rectangle. Print the result.
