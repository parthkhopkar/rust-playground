#[derive(Debug)]
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn build_user(username: String, email: String) -> User{
	User{
		email: email,
		username: username,
		active: true,
		sign_in_count: 1,
	}
}
 
fn main() {
    println!("Hello, world!");
    let user = User{
    	email: String::from("johndoe@aol.com"),
		username: String::from("johndoe"),
		active: true,
		sign_in_count: 1,
	};

	// Display information about the user
	println!("User {:?} with email {:?} has signed in {:?} time(s) and active status is: {:?}", 
		user.username, user.email, user.sign_in_count, user.active);

	// All attributes in the instance will be mut. Rust doesn't allow you to only mark a few attrs as mut
	let mut user1 = build_user(String::from("janedoe"), String::from("janedoe@aol.com"));
	println!("{:?}", user1);
}
