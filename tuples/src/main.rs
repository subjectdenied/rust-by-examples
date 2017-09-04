use std::fmt;

// use tuples as function arguments and return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // use 'let' to bind tuple members to variables
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn transpose(matrix: &Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    // tuple with bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64, 
        -1i8, -2i16, -3i32, -4i64, 
        0.1f32, 0.2f64, 'a', true);

    // values can be tumple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // tuples can be printed
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is: {:?}", pair);

    println!("the reversed pair is: {:?}", reverse(pair));

    // for one element tuples, comma is required
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // destruct tuples members
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    // activity
    println!("\nMatrix:\n{}", matrix);
    println!("\nTranspose:\n{}", transpose(&matrix));

}
