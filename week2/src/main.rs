struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    // Calculate the area of a rectangle, given its width and height.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // Create a new instance of the `Rectangle` struct, and give it a width of 30 and a height of 50.
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Print out the area of the rectangle.
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    rect1.width = 100;

    // Print out the area of the rectangle.
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 5,
        height: 5,
    };

    println!("Rect 1 can hold rect2: {}.", rect1.can_hold(&rect2));
}
