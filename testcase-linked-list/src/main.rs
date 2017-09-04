use List::*;

enum List {
    // Cons: Tuple struct that wraps an element an a pointer to the next node
    Cons(u32, Box<List>), 
    // Nil: A node that signifies the end of the linked list
    Nil, 
}

impl List {
    // create empty list
    fn new() -> List {
        Nil
    }

    // consume a list and return same list with a new element at its front
    // self is not referenced here, as we return a new list!!!
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // return the length of list
    fn len(&self) -> u32 {
        // self has to be matched, because the method needs the variant of self
        // self has type &List and *self has type List
        // matching on a concrete type T is preferred over a match on reference &T
        match *self {
            // can't take ownership of the tail, cause self is borrowed
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(), 
            // Base Case: an empty list has zero size
            Nil => 0
        }
    }

    // return representation of the list as a (heap-allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }, 
            Nil => {
                format!("Nil")
            }, 
        }
    }
}

fn main() {
    // empty linked-list
    let mut list = List::new();

    // append some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // show list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
