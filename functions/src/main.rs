struct Point {
    x: f64, 
    y: f64, 
}

// implementation of methods for struct (class)
impl Point {
    // static method
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // another static method
    fn new(x: f64, y: f64) -> Point {
        Point {x: x, y: y}
    }
}

struct Rectangle {
    p1: Point, 
    p2: Point, 
}

impl Rectangle {
    // instance method
    fn area(&self) -> f64 {
        // access properties of self with dot.notation
        let Point {x: x1, y: y1 } = self.p1;
        let Point {x: x2, y: y2 } = self.p2;

        // abs returns absolute value
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point {x: x1, y: y1 } = self.p1;
        let Point {x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // this method requires the caller to be mutable
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// Pair owns resources: 2 heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // method consumes the resources of the caller object
    // no &
    fn destroy(self) {
        // destructure self
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

fn main() {
    fizzbuzz_to(100);

    // create rectangle
    let rectangle = Rectangle {
        p1: Point::origin(), 
        p2: Point::new(3.0, 4.0), 
    };

    // instance methods are colled with dot.notation
    // &self is passed automatically
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(), 
        p2: Point::new(1.0, 1.0), 
    };

    // mutable objects can call mutable methods
    // this wouldn't work, not mutable
    //rectangle.translate(1.0, 1.0);

    // this does, as the method is mutable
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // calling this again would error, as pair is already consumed (by not passing a reference)
    //pair.destroy();

    // CLOSURES/LAMBDA
    // closures are anonymous
    // function
    fn function(i: i32) -> i32 { i + 1 }
    // annotated closure
    let closure_annotated = |i: i32| -> i32 { i + 1};
    // inferred closure
    let closure_inferred = |i| i + 1;

    let i = 1;
    // call function and closures
    println!("function: {}", function(i));
    println!("closure annotated: {}", closure_annotated(i));
    println!("closure inferred: {}", closure_inferred(i));
    // a closure taking no arguments returns an i32, return type is inferred
    let one = || 1;
    println!("closure returning one: {}", one());

}

fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n)
    }
}

fn fizzbuzz(n: u32) {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    }
    else if is_divisible_by(n, 3) {
        println!("fizz");
    }
    else if is_divisible_by(n, 5) {
        println!("buzz");
    }
    else {
        println!("{}", n);
    }
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    lhs % rhs == 0
}