struct User {
    username: String;
    email: String, 
    sign_in_count: u64,
    active: bool,
}

let user1 = User {
    email: String::from("elixirdev@gmail.com"),
    username: String::from("Gabrielshow"),
    active: true,
    sign_in_count: 1,
} //creates an immutable reference to the user struct

// if you want to create a mutable reference you can use the mut keyword
let mut user2 = User {
    email: String::from("everythinggrand25@gmail.com"),
    username: String::from("Immortan"),
    active: true,
    sign_in_count: 1,
}

user2.email = String::from("authorofawashop@gmail.com")

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
} 

// or you can use the field init shorthand when variables and field have the same Name
fn build_admin_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count,
    }
}