// an attribute to hide warnings for unused code.
#[allow(dead_code)]

// enum to classify someone
// both names and type information specify the variant:
// Engineer != Scientist and Height(i32) != Weight(i32)
// each is different and independent
enum Person {
    // an enum may either be unit-like
    Engineer, 
    Scientist, 
    Height(i32), 
    Weight(i32), 
    // or like structs
    Info { name: String, height: i32 }
}

enum Status {
    Rich, 
    Poor, 
}

enum Work {
    Civilian, 
    Soldier, 
}

// C-like
enum Number {
    Zero, 
    One, 
    Two, 
}

enum Color {
    Red = 0xff0000, 
    Green = 0x00ff00, 
    Blue = 0x0000ff, 
}

// a function which takes a Person enum as an arg and returns nothing
fn inspect(p: Person) {
    // usage of enum must cover all cases!
    // so a match is used
    match p {
        Person::Engineer => println!("Is an Engineer"), 
        Person::Scientist => println!("Is a Scientist"), 
        Person::Height(i) => println!("Has a height of {}", i), 
        Person::Weight(i) => println!("Has a weight of {}", i), 
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

fn main() {
    let person = Person::Height(18);
    let amira = Person::Weight(10);
    // to_owned creates an owned string from a string slice
    let dave = Person::Info { name: "Dave".to_owned(), height: 72 };
    let rebecca = Person::Scientist;
    let rohan = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);

    use Status::{Poor, Rich};
    use Work::*;

    // same as Status::Poor
    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"), 
        Poor => println!("The poor have no money ..."), 
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    // enums can be cast as integers
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
