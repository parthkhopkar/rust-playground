fn main() {
    let s = "hello";
    let s_mut = String::from(s);
    println!("{:?} and {}", s, s_mut);

    // Example of a move
    let s1 = String::from("Hello, World!");
    let _s2 = s1;  // Prefixed s2 with _ to get rid of the warning
    // s1 has now been invalidated

    // println!("{:?}", s1); will no longer work

    // The solution is to use clone()
    let str1 = String::from("Hello World!");
    let str2 = str1.clone();

    println!("str1 = {}, str2 = {}", str1, str2);

    // Ownership and functions
    let s_mut = takes_ownership(s_mut);  // This takes the ownership of s_mut and gives it to the function

    // Try using s again
    println!("The value of s_mut is: {:?}", s_mut); // does not work

    let x = 5;
    makes_copy(x);

    // Try using x again
    println!("The value of x is: {:?}", x); // Works!

    let a_main = String::from("Hello Universe");
    let a_main_size = reference(&a_main);
    // Trying to see if a_main is still owned by main
    println!("The value of a_main is: {:?} and its size is: {:?}", a_main, a_main_size);  // This works!

    // Mutable references
    let mut x = String::from("Hello Milky Way!");
    modify(&mut x);
    println!("The modified string is: {:?}", x);

    // You can only have one mutable reference to a piece of data in a particular scope
    let _r1 = &mut x;
    let _r2 = &mut x;

    // The compiler throws an error only when the variables are used
    // println!("r1: {:?}, r2 {:?}", r1, r2); will throw an error
    // This way Rust won't even compile code which can lead to data races!

    // A look at dangling references
    // Rust prevents any kind of dangling references
    // let &r = dangle(); throws an error


    // Exploring slice type
    let slice = first_word(&a_main);
    println!("The first word is: {:?}", slice);

} // End of main


fn takes_ownership(some_string: String) -> String{
	println!("{:?}", some_string);
	some_string
}

fn makes_copy(some_int: i32){
	println!("{:?}", some_int);
}

// Function that takes in a reference
fn reference(a: &String) -> usize{
	a.len()
}

// Function that tries to modify a reference which is not mutable does not work

/*
fn modify(s: &String){
	s.push_str(" Here I am!");
}
*/

// Use a function that has a mutable variable as input instead
fn modify(s: &mut String){
	s.push_str(" Here I am!")
}

// This function returns a dangling reference which causes issues with the Rust compiler
/*
*	fn dangle() -> &String{
*		let s = String::from("Hello")
*
*		&s
*	}
*/

// A function which returns the first word it finds in a sentence and rturns the end index
fn first_word(s: &String) -> &str{
	let bytes = s.as_bytes();

	for (i,&item) in bytes.iter().enumerate(){
		if item == b' '{
			return &s[0..i];
		}
	}
	// If no space is found after traversing the string, the length of the string is returned
	&s[..]

}