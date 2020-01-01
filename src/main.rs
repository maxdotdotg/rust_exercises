fn main() {
    let x = 5; 
    let x = x + 1; // x is shadowed, which means value of x is now 6
    let x = x + 2; // shadowed again, value of x is 8
    println!("the value of x is {}", x);
    // shadowing works like re-defining a variable, and only works
    // because it uses normal variable assignment?
    // also, shadowing allows for type-changing
    let spaces = "    "; // was as string, I think?
    let spaces = spaces.len(); // became an int
    println!("value of spaces is {}", spaces);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}",c, z, heart_eyed_cat);

    // tuples are fixed length, can't be modified
    // type annotation below for reference, it's not required
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup; // destructuring the tuple to access individual values
    println!("index test: tup index 0 is accessed with dots, so tup.0: {}", tup.0);

    // this doesn't work with the default formater
    // println!("{}", tup);
    println!("pretty print of tup: {:#?}", tup); // this is pretty-print for debugging
    println!("tup is {:?}", tup); // this is print for debugging
    println!("the vlaue of y is {}", y);

    // it's arrays, not lists
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("testing indexing: months[2] is: {}", months[2]);
}

