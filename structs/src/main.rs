#[derive(Debug)]  // This trait enables prettyprinting of the struct
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

	let user2 = User{
		username: String::from("alyxhalflife"),
		email: String::from("alyx@valve.com"),
		..user1
	};

	println!("{:?}", user2);

	// Tuple structs
	#[derive(Debug)]
	struct Point(i32, i32, i32);
	let p1 = Point(1,1,1);
	println!("{:?}", p1);

}  // End of main
