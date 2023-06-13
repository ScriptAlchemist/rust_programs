// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}
/*
Add a function rect_area which calculates the area of a Rectangle 
(try using nested destructuring).
Add a function square which takes a Point and a f32 as arguments, and returns
a Rectangle with its
top left corner on the point, and a width and height corresponding to the f32.
*/
impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Rectangle {
            top_left: Point { x: left_edge, y: top_edge },
            bottom_right: Point { x: right_edge, y: bottom_edge}
        } = self;
        let width = right_edge - left_edge;
        let height = top_edge - bottom_edge;
        width * height
    }
    fn square(point: Point, int: f32) -> Rectangle {
        let Point { x: left_edge, y: top_edge } = point;

        Rectangle {
            top_left: point,
            bottom_right: Point { x: left_edge + int, y: top_edge - int},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_area() {
        let rect = Rectangle {
            top_left: Point { x: 0.0, y: 15.0 },
            bottom_right: Point { x: 10.0, y: 0.0 },
        };
        assert_eq!(rect.rect_area(), 150.0);
    }

    #[test]
    fn test_square() {
        let point = Point { x: 0.0, y: 10.0 };
        let side = 5.0;
        let square = Rectangle::square(point, side);
        assert_eq!(square.top_left.x, 0.0);
        assert_eq!(square.top_left.y, 10.0);
        assert_eq!(square.bottom_right.x, 5.0);
        assert_eq!(square.bottom_right.y, 5.0);
    }
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: _left_edge, y: _top_edge } = point;
}











































