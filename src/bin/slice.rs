fn main() {
    // still getting used to handling this
    // definitely did it wrong the first time
    // let test_string = "hello, world"
    // which threw a compiler error:
    /*
    error[E0308]: mismatched types
     --> src/bin/slice.rs:4:31
       |
       4 |     println!("{}", first_word(&test_string));
         |                               ^^^^^^^^^^^^ expected struct `std::string::String`, found &str
           |
             = note: expected type `&std::string::String`
                          found type `&&str`

     */
    let test_string = String::from("hello, world");
    let test_string_two = "hello again";

    println!("{}", first_word(&test_string));

    let hello = &test_string[..6]; // first to 6th, exclusive
    let world = &test_string[6..]; // 6th to end
    let total_string = &test_string[..]; // start to end
    println!("doing it by hand: '{}' and '{}'", hello, world);
    println!("a slice containing the whole thing: '{}'", total_string);
    println!(
        "using slices! '{}'",
        first_word_using_slices(&test_string_two)
    );
    println!("using slices! '{}'", first_word_using_slices(&test_string));
}

fn first_word(s: &String) -> usize {
    // return index of the end of the borrowed string
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // using &item because it's what `.enumerate()` returns
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_using_slices(s: &str) -> &str {
    // taking borrowed str allows for both &str and &String
    // since both can be slices
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
