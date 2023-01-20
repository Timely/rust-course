struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: Rectangle) -> bool {
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

    println!("The width has now changed and is {}", rect1.width);

    let rect2: Rectangle = Rectangle {
        width: 1,
        height: 1,
    };

    let rect3: Rectangle = Rectangle {
        width: 999,
        height: 999,
    };

    print!("Can rect1 hold rect2? {}! ", rect1.can_hold(rect2));
    print!("Can rect1 hold rect3? {}! ", rect1.can_hold(rect3));
}
