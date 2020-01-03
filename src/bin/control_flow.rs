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

    let mut counter = 0; // counter is mutable because it'll change!

    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter * 2 // return results and 
                              // break are on the same line
        }
    };

    println!("the result of the loop is {}", result);

    // while loop!
    let mut while_counter = 3;

    while while_counter != 0 { // condition
        println!("{}!", while_counter);
        while_counter -=1; //modify counter
    }
    println!("liftoff!");
}
