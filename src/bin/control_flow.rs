fn main() {
    let number = 3;

    if number < 5 
    // this is very wrong from the rustfmt perspective
    // but I think it's a helpful demo of statements vs expressions
    { println!("first condition was true"); // statement
    } // true arm of the if-expression, returning the statement
    else 
    { println!("first condition was false"); // statement
    } // false arm of the if-expression, returning the statement

    // else ifs yo
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // we can also use if in variable assignment
    // worth noting that types have to match
    let condition = true;
    let new_number = if condition {
        5 // expression, returns 5
    } else {
        6 // expression returns 6
    }; // statement, assigns a value to new_number

    println!("the value of new_number is {}", new_number);
}
