use std::fmt; // import fmt 

// a structure holding 2 numbers (min, max)
#[derive(Debug)]
struct MinMax(i64, i64);

// Display implementation for MinMax
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use self.number to refer to each positional data point
        write!(f, "({}, {})", self.0, self.1)
    }
}

// structure where fields are nameable for comparison
#[derive(Debug)]
struct Point2D {
    x: f64, 
    y: f64, 
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // custom so only x and y are used
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = self.x;
        let y = self.y;
        write!(f, "{:b}", x as i64 + y as i64)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structrues:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small range is {small}", 
        small = small_range, 
        big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // binary needs another trait impl for fmt::Binary
    println!("What does Point2D look like in binary: {:b}?", point);
}
