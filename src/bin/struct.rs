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

    let user4 = User {
        email: String::from("third@third.co"),
        username: String::from("num3"),
        ..user1 // use the remaining values from user1
    };

    println!("user4 active? {:?}", user4.active);

    // same as user4, just spelled out using values from another instance of the struct
    let user5 = User {
        email: String::from("someone-again@example.com"),
        username: String::from("fiph"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // tuple structs are structs without named fields
    // I'm guessing they're used for convenience and readability
    struct Color(i32, i32, i32);

    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let point_var = Point(0, 0, 0);
    let color_var = Color(0, 0, 0);

    println!("{:?}", point_var);
}

// structs are types of a sort, I think?
// yes, they're custom data types
// that means we can return an instance of it
fn build_user(email: String, username: String) -> User {
    // take an email and username to build an instance of
    // the User type (defined in the struct block above)
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
