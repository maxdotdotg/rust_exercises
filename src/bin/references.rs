fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // a reference to s1
                                     // "These ampersands are references, and they allow you to refer
                                     // to some value without taking ownership of it"
                                     // ch04-02-references-and-borrowing
    println!("length of '{}' is {}", s1, len);

    let mut s = String::from("hello");
    println!("value of s is '{}' before calling `change`", s);
    change(&mut s); //mutable reference because it's being changed!
    println!("value of s after mutation is '{}'", s);

    let r1 = &s;
    let r2 = &s;
    println!("'{}' and '{}' are references to s", r1, r2);
    // r1 and r2 are no longer used
    let _r3 = change(&mut s); // r3 is unused, and alters s
    println!("after calling `change` again, the value of s is '{}'", s);
}

fn calculate_length(s: &String) -> usize {
    // borrows (takes a reference to) a string
    // returns an int of type matching the os architecture?
    s.len() // returned, so no semicolon
            // since s is a reference, it's not out of scope when the function exits
}

fn change(some_string: &mut String) {
    // borrows a mutable reference to String
    // the reference has to be mutable because we're changing the data itself
    // not using the data as-is for another operation
    // like we are for calculate_length
    some_string.push_str(", world");
    // "you can have only one mutable reference to a particular
    // piece of data in a particular scope."
    // ch04-02-references-and-borrowing
    // ALSO
    // can't have mutable and immutable references while
    // immutable references are still used / in scope
    // no way to guarantee data content, can't eliminate data races
}
