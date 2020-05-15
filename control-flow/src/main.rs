use std::io;

fn main() {
	// Get number as input
	let mut num = String::new();
	println!("Enter a number:");
	io::stdin().read_line(&mut num).expect("Failed to read");
	let num: u32 = match num.trim().parse(){
		Ok(n) => n,
		Err(_) => 1,
	};

	let another_num = if num > 5 {10} else {100};
	println!("Another number is: {:?}", another_num);

	// Loops
	
}
