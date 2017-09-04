fn main() {
    // binding lives in main function
    let long_lived_binding = 1;

    // a block, with a smaller scope than main
    {
        // exists within the block only
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        // shadowing the outer binding
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }

    // error -> short lived binding from within scope doesn't exist here
    //println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // shadowing the prev binding
    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);

    // declare a variable binding
    let a_binding;
    {
        let x = 2;

        // initialize the binding
        a_binding = x*x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // error -> usage of uninitialized binding
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    // this works as var is initialized now
    println!("another binding: {}", another_binding);
}
