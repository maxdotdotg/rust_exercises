// match!
// this is one of the coolest things about rust that I've seen

// There are four kinds of Coin
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // some quarters were minted with states
}

// 50 states, abbreviated here
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc etc etc
}

// we're creating a function using match, which requires all cases
// (in this case, kinds of Coin) to be addressed
// match arms can include further computation, or in this case,
// printing a line to stdout
// AND BE SURE TO INCLUDE THE RETURN TYPE! YOU FORGOT!
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1 // last value is returned
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}", state);
            25
        }
    }
}

fn main() {
    // define first as a kind of Coin
    let first = Coin::Penny;

    // call value_in_cents, which matches the kind of Coin
    println!("{}", value_in_cents(first));

    // create quarter, type Quarter, which requires the enum UsState
    // to be passed as well
    let quarter = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(quarter));
}
