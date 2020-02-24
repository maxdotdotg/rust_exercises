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


    let some_u8_value = Some(0u8);

    // if-let condenses this match expression...
    match some_u8_value {
       Some(3) => println!("three"),
       _ => (),
    }

    // into this
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // you can think of if let as syntax sugar for a match that runs code
    // when the value matches one pattern and then ignores all other values.
    // ch06-03-if-let

}

