// globals are declared outside of all scopes
// const: unchangeable value
// static: possibly mut(able) with 'static lifetimie
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // access constants in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // access constants in main-thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is: {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // error const cannot be modified
    //THRESHOLD = 5;
}
