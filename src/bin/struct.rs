// define User struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {
    // now let's use it!
    // create an instance of the User struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    // values are accessible with dot notation
    println!("{:?}", user1.email);


    // structs can be mutable too!
    let mut user2 = User {
        email: String::from("someonetwo@example.com"),
        username: String::from("someonetwo"),
        active: true,
        sign_in_count: 1,
    };
    println!("before modification: {}", user2.email);
    user2.email = String::from("anotherworld@anothertime");
    println!("after modification: {}", user2.email);
    

    let username3 = String::from("user3");
    let email3 = String::from("somethingelse@stuff.co");
    
    let user3 = build_user(email3, username3);
    println!("user3 email: {}, username: {}", user3.email, user3.username);

}

// structs are types of a sort, I think?
// that means we can return an instance of it
fn build_user(email: String, username: String) -> User {
    User {
        // email: email,
        // username: username, 
        // using field init shorthand below
        // since struct field and variable names match
        email, 
        username,
        active: true, 
        sign_in_count: 1,
    }
}
