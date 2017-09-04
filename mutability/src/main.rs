fn main() {
    let _immutable_binding = 1; // underscore at start of variable name hides "not used" warning
    let mut mutable_binding = 1;

    println!("before mutation: {}", mutable_binding);

    // this is ok
    mutable_binding += 1;
    println!("after mutation: {}", mutable_binding);

    // modifying an immutable var panics
    // _immutable_binding +=1;
}
