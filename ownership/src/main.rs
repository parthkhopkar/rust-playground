fn main() {
    let s = "hello";
    let s_mut = String::from(s);
    println!("{:?} and {}", s, s_mut);

    // Example of a move
    let s1 = String::from("Hello, World!");
    let s2 = s1;
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
}

fn takes_ownership(some_string: String) -> String{
	println!("{:?}", some_string);
	some_string
}

fn makes_copy(some_int: i32){
	println!("{:?}", some_int);
}