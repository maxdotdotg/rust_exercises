// match!
// this is one of the coolest things about rust that I've seen

// There are four kinds of Coin
enum Coin {
    Penny,
    Nickel, 
    Dime, 
    Quarter,
}

// we're creating a function using match, which requires all cases 
// (in this case, kinds of Coin) to be addressed
// AND BE SURE TO INCLUDE THE RETURN TYPE! YOU FORGOT!
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1, 
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


fn main() {
    // define first as a kind of Coin
    let first = Coin::Penny;

    // call value_in_cents, which matches the kind of Coin
    // and returns an int
    println!("{}", value_in_cents(first));

}
