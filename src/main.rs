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


}

