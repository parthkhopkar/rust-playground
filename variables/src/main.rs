fn main() {
	const PI: f64 = 3.14;
    let mut x = 10;
    println!("The value of x is {}", x);
    x = 123;
    println!("The value of x now is {}", x);
    const MAX_VAL: u32 = 100000;
    println!("MAX_VAL is {}", MAX_VAL);
    println!("PI is {}", PI);

    let guess: u32 = "45".parse().expect("Not a number");
    println!("{}", guess);

    let s = "Hello World!";
    println!("{}", s);

    // Floating point numbers
    let float_num64 = 5.3/2.4;

    let float_num32: f32 = 5.3/2.4;

    println!("Float number with f32: {}", float_num32);

    println!("Float number with f64 {}", float_num64);

    // Booleans
    let bool_value = false;
    if bool_value {
        println!("Bool value is true"); 
    }
    else {
    	println!("Bool value is false");
    }

    // Character variables
    let emoji = '😻';
    println!("This is a unicode emoji: {}", emoji);

    // Tuples
    let tup = (500, 'a', true);
    println!("A tuple looks like this in Rust: {:?}", tup);
    // Printing the elements of the tuple in a loop
    // for val in tup.iter(){
    // 	println!("{:?}", val);
    // }

    // So, that doesn't work, let's try accessing individual elements of the array
    println!("The elemets of the tuple are: {}, {}, {}", tup.0, tup.1, tup.2);

    // Arrays
    let arr = [1, 2, 3, 4, 5];
    let index = 10;
    println!("The array looks like this: {:?}", arr);
    println!("The 10th element in the array is: {:?}", arr[index]);

    // Converting a string to int
    let guess: u32 = "42".parse().expect("Need a number");

    println!("The value of guess is: {:?}", guess);
}