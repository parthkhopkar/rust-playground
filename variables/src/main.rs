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
}
