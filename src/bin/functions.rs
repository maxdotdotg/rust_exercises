fn main() {
    let merf = 66; // this is a statement, it doesn't return a value
    another_function(35); // this is an expression, it DOES return a value
    println!("{}", merf); // also an expression?
                          // "Calling a function is an expression. Calling a macro is an expression"
                          // from ch03-03-how-functions-work
    nested_expression();

    let z = with_return_values();
    println!("the value of z is {}", z);

    let a = plus_one(5);
    println!("the value of a is {}", a);
}

// declare input type upfront, static typing
// in python, no typing
// def another_function(x):
//     print("value of x is {}".format(x))
fn another_function(x: i32) {
    println!("value of x is {}", x); // I guess this counts as return values?
}

fn nested_expression() {
    let x = 5;
    let y = {
        let x = 3; // statement, just variable binding
        x + 1 // expression, returning a value, also has no semicolon
    }; // a statement because it's assigned to a variable?

    println!("the value of y is {}", y);
}

// cast the return type, and use arrows!
fn with_return_values() -> i32 {
    // "In Rust, the return value of the function is synonymous with the value of the final
    // expression in the block of the body of a function."
    // from ch03-03-how-functions-work
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
