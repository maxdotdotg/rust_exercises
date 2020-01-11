fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // a reference to s1
    // "These ampersands are references, and they allow you to refer 
    // to some value without taking ownership of it"
    // ch04-02-references-and-borrowing
    println!("length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // borrows (takes a reference to) a string
    // returns an int of type matching the os architecture?
    s.len() // returned, so no semicolon
}
