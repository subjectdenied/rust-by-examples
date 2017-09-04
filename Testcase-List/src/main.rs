use std::fmt;

// struct named List containing a Vector
struct List(Vec<i32>);

impl fmt::Display for List { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // extract value using tuple indexing
        // and create ref to vec
        let vec = &self.0;
        write!(f, "[")?;

        // iterate over vec, while enumerating the iteration 
        // count in count
        for (count, v) in vec.iter().enumerate() {
            // for every element except the first, add a comma
            // use the ? operator, or try! macro, to return on errors
            if count !=0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1,2,3]);
    println!("{}", v);
}
