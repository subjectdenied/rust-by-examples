use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // fixed size array (type is same)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // all elements can be initialized with the same value
    let ys: [i32; 500] = [0; 500];

    // index starts with 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // len returns size of array
    println!("array size: {}", xs.len());

    // array are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // array can be automatically borrowed as slices
    println!("borrow the whole array as slice");
    analyze_slice(&xs);

    // slices can point to a section of an array
    analyze_slice(&ys[1..4]);

    // out of bound indexing yields a panic!
    println!("{}", xs[5]);
}
