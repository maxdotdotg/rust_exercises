fn main () {
    let merf = 66; // this is a statement, it doesn't return a value
    println!("{}", merf);
    another_function(35); // this is an expression, it DOES return a value
}

// declare input type upfront, static typing
// in python, no typing
// def another_function(x):
//     print("value of x is {}".format(x))
fn another_function (x: i32) {
    println!("value of x is {}", x); // I guess this counts as return values?
}
