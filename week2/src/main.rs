struct Rectangle {
    pub width: u32,
    pub height: u32,
}

// Calculate the area of a rectangle, given its width and height.
fn area(rectangle: Rectangle) -> u32 {
    todo!()
}

fn main() {
    // Create a new instance of the `Rectangle` struct, and give it a width of 30 and a height of 50.
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Print out the area of the rectangle.
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}
