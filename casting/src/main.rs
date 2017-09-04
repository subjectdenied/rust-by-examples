// supresses warnings from casts with overflows
#![allow(overflowing_literals)]
// type aliases
// camelCase
type NanoSecond = u64;
type Inch = u64;

// Use an attribute to silence warnings about non camelCase type naming.
#[allow(non_camel_case_types)]
type u64_t = u64;


fn main() {
    let decimal = 65.4321_f32;

    // error -> no implicit conversion
    //let integer: u8 = decimal;

    // explizit conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("casting: {} -> {} -> {}", decimal, integer, character);

    // when castding any value to an usigned type, T, 
    // std::T::Max + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // first 8 least significant bits (LSB) are kept
    // but rest towards the most significant bit (MSB) get truncated
    println!("1000 as u8 is: {}", 1000 as u8);
    // -1 + 256 = 255
    println!("-1 as u8 is: {}", (-1i8) as u8);
    // for positive numbers, this is the same as the modulus
    println!("1000 mod 256 is: {}", 1000 % 256);

    // when casting to a signed type, the bitwise result is the same as
    // first castding to the corresponding unsigned type
    // if the most significant bit of that value is 1, then the value is negative

    // unless it already fits, of course
    println!("128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    println!("128 as a i8 is: {}", 128 as i8);

    // repeating the above example
    // 1000 as u8 -> 232
    println!("1000 as a i8 is: {}", 1000 as i8);
    // and the two's complement of 232 is -24
    println!("232 as a i8 is: {}", 232 as i8);

    // INFERENCE
    // annotation
    let elem = 5u8;

    // empty vector (growable array)
    let mut vec = Vec::new(); // == Vec<_>

    // insert an element
    // if this statement is commented, the compiler would not know
    // which type the vector contains -> error
    vec.push(elem);

    println!("{:?}", vec);

    // ALIASES
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!("{} nanoseconds + {} inches = {} unit=?", 
        nanoseconds, 
        inches, 
        nanoseconds + inches);

}
