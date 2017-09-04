#[derive(Debug)]
struct Person<'a> {
    name: &'a str, 
    age: u8
}

// a unit struct
struct Nil;

// a tuple struct
struct Pair(i32, f32);

// a struct with two fields
#[derive(Debug)]
struct Point {
    x: f32, 
    y: f32,
}

// structs can be reused as fields of another struct (Dictionaries?)
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point, 
    p2: Point, 
}

fn rect_area(rect: &Rectangle) -> f32 {
    let p1:f32 = rect.p2.x - rect.p1.x;
    let p2:f32 = rect.p2.y - rect.p1.y;
    p1 * p2
}

fn square(point: &Point, size: f32) -> Rectangle {
    let p1 = Point {x: point.x - size, y: point.y};
    let p2 = Point {x: p1.x + size, y: p1.y + size};
    Rectangle {p1: p1, p2: p2}
}

fn main() {
    // create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};

    // debug struct
    println!("{:?}", peter);

    // instantiate a point
    let point: Point = Point { x: 0.3, y: 0.4 };

    // access fields of point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // destructure the point using a 'let' binding
    let Point {x: my_x, y: my_y} = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point {x: my_y, y: my_x}, 
        p2: point,
    };

    // instantiate a unit struct
    let _nil = Nil;

    // instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // access fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // activity
    // calc rectangle area
    let rectangle = Rectangle {
        p1: Point {x: 1.0, y: 1.0}, 
        p2: Point {x: 4.0, y: 5.0},
    };
    println!("rectangle area is: {}", rect_area(&rectangle));

    // calc square
    let origin_lower_left = Point{x: 5.0, y: 0.0};
    let square = square(&origin_lower_left, 4.0);
    println!("square is: {:?}", square);
}
