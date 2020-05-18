fn main() {
	let num = 10;
    println!("Hello, world!");
    assing_value(num);
    println!("Double value of num is: {:?}", double_num(num));
}

fn assing_value(x: i32){
	let y = {
		let a = x;
		a + 1
	};
	println!("The value assigned to y is: {:?}", y);
}

// Function with a return statement
fn double_num(x: i32) -> i32{
	x*2
}

/*
* This is a multi-line comment
*/