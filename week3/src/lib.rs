use std::any::TypeId;

enum Shape {
    Circle { radius: f64 },
    Square { length: f64 },
    Rectangle { width: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => radius * radius * std::f64::consts::PI,
        Shape::Square { length } => length * length,
        Shape::Rectangle { width, height } => width * height,
    }
}

#[test]
fn test_shape_areas() {
    assert_eq!(area(&Shape::Circle { radius: 1.0 }), std::f64::consts::PI);
    assert_eq!(area(&Shape::Square { length: 2.0 }), 4.0);
    assert_eq!(
        area(&Shape::Rectangle {
            width: 3.0,
            height: 8.0
        }),
        24.0
    );
}

fn random_shape() -> Shape {
    match rand::random::<i32>() % 3 {
        0 => Shape::Circle {
            radius: rand::random(),
        },
        1 => Shape::Square {
            length: rand::random(),
        },
        _ => Shape::Rectangle {
            width: rand::random(),
            height: rand::random(),
        },
    }
}

fn circle_area(shape: &Shape) -> Option<f64> {
    if let Shape::Circle { .. } = shape {
        Option::Some(area(shape))
    } else {
        Option::None
    }
}

#[test]
fn test_circle_area() {
    assert_eq!(
        circle_area(&Shape::Circle { radius: (1.0) }),
        Some(std::f64::consts::PI)
    );
    assert_eq!(
        circle_area(&Shape::Rectangle {
            width: 1.0,
            height: 1.0
        }),
        None
    );
    assert_eq!(circle_area(&Shape::Square { length: 1.0 }), None);
}
