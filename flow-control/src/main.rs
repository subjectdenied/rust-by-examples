enum Color {
    Red, 
    Blue, 
    Green, 
    RGB(u32, u32, u32), 
    HSV(u32, u32, u32), 
    HSL(u32, u32, u32), 
    CMY(u32, u32, u32),  
    CMYK(u32, u32, u32, u32), 
}

fn age() -> u32 {
    15
}

fn fn_match() {
    // simply match
    let number = 13;
    println!("tell me about {}", number);

    match number {
        1 => println!("One!"), 
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"), 
        // match inclusive range
        13...19 => println!("a teen"), 
        // handle rest of cases
        _ => println!("nothing special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0, 
        true => 1, 
    };

    println!("{} -> {}", boolean, binary);

    // destructuring tuples
    let pair = (0, -2);
    println!("tell me about {:?}", pair);
    match pair {
        (0, y) => println!("first is '0' and 'y' is '{:?}'", y), 
        (x, 0) => println!("'x' is '{:?}' and last is '0'", x), 
        _ => println!("it doesn't matter what they are"), 
    }

    // destructuring enums
    let color = Color::RGB(122, 17, 40);
    println!("what color is it?");
    match color {
        Color::Red => println!("The color is Red!"), 
        Color::Blue => println!("The color is Blue!"), 
        Color::Green => println!("The color is Green!"), 
        Color::RGB(r, g, b) => 
            println!("red: {}, green: {}, and blue: {}!", r, g, b), 
        Color::HSV(h, s, v) => 
            println!("hue: {}, saturation: {}, value: {}!", h, s, v), 
        Color::HSL(h, s, l) => 
            println!("hue: {}, saturation: {}, lightness: {}!", h, s, l), 
        Color::CMY(c, m, y) => 
            println!("cyan: {}, magenta: {}, yellow: {}!", c, m, y), 
        Color::CMYK(c, m, y, k) => 
            println!("cyon: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
    }

    // pointers/ref
    // dereferencing uses *
    // destructuring uses &, ref, and ref mut
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val)
    }

    // no reference
    let _not_a_reference = 3;
    // reference with ref
    let ref _is_a_reference = 3;
    // by defining 2 values without references, references can be retrieved via (mut) ref
    let value = 5;
    let mut mut_value = 6;
    // use ref to create a reference
    match value {
        ref r => println!("Got a reference to a value: {:?}", r), 
    }

    // ref mut
    match mut_value {
        ref mut m => {
            // we have a reference, needs to be dereferenced before usage
            *m += 10;
            println!("we added 10, 'mut_value: {:?}", m);
        }
    }

    // structs
    struct Foo {x: (u32, u32), y: u32 }
    // destructure members
    let foo = Foo {x: (1,2), y: 3};
    let Foo {x: (a, b), y } = foo;
    println!("a = {}, b = {}, y = {}", a, b, y);

    // destructuring structs allows renaming of vars
    // order is not important
    let Foo {y: i, x: j} = foo;
    println!("i = {:?}, j = {:?}", i, j);
    // also vars can be ignored
    let Foo {y, ..} = foo;
    println!("y = {}", y);
    // this errors as x is not mentioned
    //let Foo { y } = foo;

    // match guards
    let pair = (2, -2);
    println!("tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("twins!"), 
        (x, y) if x + y == 0 => println!("zero!"),
        (x, _) if x % 2 == 1 => println!("first one is odd!"), 
        _ => println!("no correlation!"),   
    }

    // binding
    println!("tell me about the type of person you are");
    match age() {
        0 => println!("i was just born, i guess!"),  
        // bind a var to the match
        n @ 1 ... 12 => println!("i'm a child of age {:?}", n), 
        n @ 13 ... 19 => println!("i'm a teen of age {:?}", n), 
        n => println!("i'm an old person of age {:?}", n), 
    }
}

fn fn_iflet() {
    let number = Some(7); // Option<i32>
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("matched {:?}!", i);
    }

    // else
    if let Some(i) = letter {
        println!("matched {:?}!", i);
    } else {
        // letter == None so this code is called
        println!("didn't match a number. let's go with a letter");
    }

    // else if 
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("matched: {:?}!", i);
    } else if i_like_letters {
        println!("didn't match a number, let's go with a letter!");
    } else {
        println!("i don't like letters, let's go with an emoticon!");
    }
}   

fn fn_whilelet() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("> 9, quit!");
            optional = None;
        } else {
            println!("i is {:?}. try again!", i);
            optional = Some(i + 1)
        }
    }
}

fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            // this expression return an i32
            10 * n
        } else {
            println!(", and is a big number, reduce by two");
            n / 2
        }; // semicolon is needed here

    println!("{} -> {}", n, big_n);

    // LOOP
    let mut count = 0u32;
    println!("Let's count until infinity!");

    // infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // exit loop
            break;
        }
    }

    // NESTED LOOP AND LABELS (break specific named loop)
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // this would break only the inner loop
            //break;

            // break the outer loop
            break 'outer;
        }

        println!("This point will never be reached!");
    }

    println!("Exited the outer loop");

    // RETURNING FROM LOOPS
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    // WHILE
    // fizzbuzz
    let mut c = 1;

    while c < 101 {
        if c % 15 == 0 {
            println!("fizzbuzz");
        } else if c % 3 == 0 {
            println!("fizz");
        } else if c % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", c);
        }

        c += 1;
    }

    // FOR AND RANGE
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // MATCH
    fn_match();

    // IF LET
    fn_iflet();

    // WHILE LET
    fn_whilelet();
}
