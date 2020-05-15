// use std::io;

fn main() {
	// Get number as input
	let num = 34;
	// println!("Enter a number:");
	// io::stdin().read_line(&mut num).expect("Failed to read");
	// let num: u32 = match num.trim().parse(){
	// 	Ok(n) => n,
	// 	Err(_) => 1,
	// };

	let another_num = if num > 5 {10} else {100};
	println!("Another number is: {:?}", another_num);

	// Returining values from loops
	let mut counter = 0;

	let result = loop {
		counter += 1;

		if counter == 20{
			break counter*2;
		}
	};

	println!("The value of result after the loop is: {:?}", result);

	// While loop that counts multiples of 13
	let num = 13;
	let mut multiplier = 1;
	while multiplier <= 10 {
		println!("{} X {} = {}", num, multiplier, num*multiplier);
		multiplier += 1;
	 } 

	// While loop that calculates factorial
	let mut fact_num = 5;
	let mut fact_result = 1;
	while fact_num >=1 {
		fact_result = fact_num * fact_result;
		fact_num -= 1;
	}

	println!("The result of factorial is: {:?}", fact_result);

	// Iterating over array using for
	let arr = [3.14, 45.0, 52.0, 9.0, 76.7];

	print!("The elements of the array are: ");
	for element in arr.iter(){
		print!("{:?} ", element);
	}

	println!();

	// for loop in a range
	print!("For loop in a range: ");
	for i in (1..11).rev(){
		print!("{:?} ", i);
	}

	println!();

	// Convert temp from F to C
	let temprature_in_f = 64.0;
	let temprature_in_c = (temprature_in_f - 32.0)*(5.0/9.0);
	println!("{} F is equal to {} C", temprature_in_f, temprature_in_c);

	// nth Fibonacci number
	let n = 4;
	println!("The {}th Fibonacci number is: {:?}", n, fibonacci(n));

}

fn fibonacci(n: u32) -> u32{
	let mut a = 0;
	let mut b = 1;
	if n == 1{
		return a
	}
	else if n == 2{
		return b
	}
	else{
		let mut result = 0;
		for _ in 2..n{
			result = a + b;
			a = b;
			b = result;
		}
		return result
	}

}