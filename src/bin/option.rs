fn plus_one(x: Option<i32>) -> Option<i32> {
    // reminder, matches are exhaustive
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // this gives us access to the value
        // contained in the Option
        // which is still something I do not understand
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
